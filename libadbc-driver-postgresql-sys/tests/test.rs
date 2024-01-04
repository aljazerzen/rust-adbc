use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

use libadbc_driver_postgresql_sys::*;

#[test]
fn basic() {
    let mut error: MaybeUninit<AdbcError> = MaybeUninit::uninit();

    let mut database: MaybeUninit<AdbcDatabase> = MaybeUninit::uninit();
    unsafe {
        // This is required because AdbcDatabaseNew requires private_data to be null,
        // since that means uninitialized.
        (*database.as_mut_ptr()).private_data = std::ptr::null_mut();

        AdbcDatabaseNew(database.as_mut_ptr(), error.as_mut_ptr());
    }
    check_for_error(&mut error).unwrap();

    unsafe {
        let uri_name = CString::new("uri").unwrap();
        let uri_value = CString::new("postgresql://root:root@localhost:5432/dummy").unwrap();
        AdbcDatabaseSetOption(
            database.as_mut_ptr(),
            uri_name.as_ptr(),
            uri_value.as_ptr(),
            error.as_mut_ptr(),
        );
    }
    check_for_error(&mut error).unwrap();

    unsafe {
        AdbcDatabaseInit(database.as_mut_ptr(), error.as_mut_ptr());
    }
    check_for_error(&mut error).unwrap();

    // connection

    let mut connection: MaybeUninit<AdbcConnection> = MaybeUninit::uninit();
    unsafe {
        AdbcConnectionNew(connection.as_mut_ptr(), error.as_mut_ptr());
    }
    check_for_error(&mut error).unwrap();

    unsafe {
        AdbcConnectionInit(
            connection.as_mut_ptr(),
            database.as_mut_ptr(),
            error.as_mut_ptr(),
        );
    }
    check_for_error(&mut error).unwrap();

    // statement

    let mut statement: MaybeUninit<AdbcStatement> = MaybeUninit::uninit();
    unsafe {
        AdbcStatementNew(
            connection.as_mut_ptr(),
            statement.as_mut_ptr(),
            error.as_mut_ptr(),
        );
    };

    // stream
    let mut stream: MaybeUninit<ArrowArrayStream> = MaybeUninit::uninit();
    let mut rows_affected: i64 = 0;

    unsafe {
        let query = CString::new("SELECT name FROM artists").unwrap();
        AdbcStatementSetSqlQuery(statement.as_mut_ptr(), query.as_ptr(), error.as_mut_ptr())
    };
    check_for_error(&mut error).unwrap();

    unsafe {
        AdbcStatementExecuteQuery(
            statement.as_mut_ptr(),
            stream.as_mut_ptr(),
            &mut rows_affected,
            error.as_mut_ptr(),
        )
    };
    check_for_error(&mut error).unwrap();

    println!("rows_affected: {rows_affected}");

    let schema: MaybeUninit<ArrowSchema> = MaybeUninit::uninit();

    unsafe {
        let get_schema = (*stream.as_ptr()).get_schema.unwrap();

        (stream.as_mut_ptr(), schema.as_mut_ptr());
        // TODO: CHECK_STREAM
    }

    let buf = [0 as libc::c_char; 1024];
    unsafe {
        // TODO: ArrowSchemaToString comes from nanoarrow, which we'd have to create bindings for
        // ABANDONED HERE <------------
        ArrowSchemaToString(
            schema.as_mut_ptr(),
            &mut buf,
            1024,
            /*recursive=*/ 1,
        );
    }
    let buf_str = CString::new(&buf).unwrap();
    println!("{}", buf_str.to_str().unwrap());

    // TODO: remaining stuff to transpile
    // while (true) {
    // ArrowArray batch = {};
    // CHECK_STREAM(stream, stream.get_next(&stream, &batch));

    // if (batch.release == nullptr) {
    //     // Stream has ended
    //     break;
    // }
    // ArrowArrayView view = {};
    // ArrowArrayViewInitFromSchema(&view, &schema, nullptr);
    // ArrowArrayViewSetArray(&view, &batch, nullptr);
    // std::cout << "Got a batch with " << batch.length << " rows" << std::endl;
    // for (int64_t i = 0; i < batch.length; i++) {
    //     std::cout << "THEANSWER[" << i
    //             << "] = " << view.children[0]->buffer_views[1].data.as_int64[i]
    //             << std::endl;
    // }
    // ArrowArrayViewReset(&view);
    // }

    // std::cout << "Finished reading result set" << std::endl;
    // stream.release(&stream);

    // CHECK_ADBC(AdbcStatementRelease(&statement, &error));
    // CHECK_ADBC(AdbcConnectionRelease(&connection, &error));
    // CHECK_ADBC(AdbcDatabaseRelease(&database, &error));
    // return EXIT_SUCCESS;
}

fn check_for_error(error: &mut MaybeUninit<AdbcError>) -> Result<(), String> {
    let error_ptr = unsafe { *error.as_ptr() };

    if error_ptr.message != std::ptr::null_mut() {
        let message = unsafe { c_str_to_string(error_ptr.message) };
        Err(message)
    } else {
        Ok(())
    }
}

unsafe fn c_str_to_string(c_str: *const libc::c_char) -> String {
    CStr::from_ptr(c_str).to_string_lossy().into_owned()
}
