cmake_minimum_required(VERSION 3.18)

project(a
        LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_compile_options(-Wall -Wextra -pedantic -Werror)

find_package(nanoarrow REQUIRED)
find_package(AdbcDriverPostgreSQL REQUIRED)
# find_package(AdbcDriverSQLite REQUIRED)

add_executable(quickstart quickstart.cc)
target_include_directories(quickstart SYSTEM PRIVATE ${nanoarrow_SOURCE_DIR}/dist)
target_link_libraries(quickstart
    PRIVATE AdbcDriverPostgreSQL::adbc_driver_postgresql_shared
    # PRIVATE AdbcDriverSQLite::adbc_driver_sqlite_static
    nanoarrow
)
add_test(NAME quickstart COMMAND a)
