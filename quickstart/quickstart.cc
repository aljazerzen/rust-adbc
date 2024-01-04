// For EXIT_SUCCESS
#include <cstdlib>
// For strerror
#include <cstring>
#include <iostream>

#include <adbc.h>
#include <nanoarrow.h>

// Error-checking helper for ADBC calls.
// Assumes that there is an AdbcError named `error` in scope.
#define CHECK_ADBC(EXPR)                                          \
  if (AdbcStatusCode status = (EXPR); status != ADBC_STATUS_OK) { \
    if (error.message != nullptr) {                               \
      std::cerr << error.message << std::endl;                    \
    }                                                             \
    return EXIT_FAILURE;                                          \
  }

// Error-checking helper for ArrowArrayStream.
#define CHECK_STREAM(STREAM, EXPR)                            \
  if (int status = (EXPR); status != 0) {                     \
    std::cerr << "(" << std::strerror(status) << "): ";       \
    const char* message = (STREAM).get_last_error(&(STREAM)); \
    if (message != nullptr) {                                 \
      std::cerr << message << std::endl;                      \
    } else {                                                  \
      std::cerr << "(no error message)" << std::endl;         \
    }                                                         \
    return EXIT_FAILURE;                                      \
  }

int main() {
  AdbcError error = {};

  // Ignoring error handling
  struct AdbcDatabase database;
  database.private_data = nullptr;
  CHECK_ADBC(AdbcDatabaseNew(&database, &error));

//   CHECK_ADBC(AdbcDatabaseSetOption(&database, "driver", "adbc_driver_postgresql", &error));
  CHECK_ADBC(AdbcDatabaseSetOption(&database, "uri", "postgresql://root:root@localhost:5432/dummy", &error));

  CHECK_ADBC(AdbcDatabaseInit(&database, &error));

  AdbcConnection connection = {};
  CHECK_ADBC(AdbcConnectionNew(&connection, &error));
  CHECK_ADBC(AdbcConnectionInit(&connection, &database, &error));

  AdbcStatement statement = {};
  CHECK_ADBC(AdbcStatementNew(&connection, &statement, &error));

  struct ArrowArrayStream stream = {};
  int64_t rows_affected = -42;

  CHECK_ADBC(AdbcStatementSetSqlQuery(&statement, "SELECT name FROM artists", &error));
  CHECK_ADBC(AdbcStatementExecuteQuery(&statement, &stream, &rows_affected, &error));

  std::cout << "Got " << rows_affected << " rows" << std::endl;

  ArrowSchema schema = {};
  CHECK_STREAM(stream, stream.get_schema(&stream, &schema));

  char buf[1024] = {};
  ArrowSchemaToString(&schema, buf, sizeof(buf), /*recursive=*/1);
  std::cout << buf << std::endl;

  while (true) {
    ArrowArray batch = {};
    CHECK_STREAM(stream, stream.get_next(&stream, &batch));

    if (batch.release == nullptr) {
      // Stream has ended
      break;
    }
    ArrowArrayView view = {};
    ArrowArrayViewInitFromSchema(&view, &schema, nullptr);
    ArrowArrayViewSetArray(&view, &batch, nullptr);
    std::cout << "Got a batch with " << batch.length << " rows" << std::endl;
    for (int64_t i = 0; i < batch.length; i++) {
      std::cout << "THEANSWER[" << i
                << "] = " << view.children[0]->buffer_views[1].data.as_int64[i]
                << std::endl;
    }
    ArrowArrayViewReset(&view);
  }

  std::cout << "Finished reading result set" << std::endl;
  stream.release(&stream);

  CHECK_ADBC(AdbcStatementRelease(&statement, &error));
  CHECK_ADBC(AdbcConnectionRelease(&connection, &error));
  CHECK_ADBC(AdbcDatabaseRelease(&database, &error));
  return EXIT_SUCCESS;
}