report = {}
valid = false

sqli_errors = {
  "SQL syntax.*?MySQL",
  "Warning.*?\\Wmysqli?_",
  "MySQLSyntaxErrorException",
  "valid MySQL result",
  "check the manual that (corresponds to|fits) your MySQL server version",
  "Unknown column '[^ ]+' in 'field list'",
  "MySqlClient\\.",
  "com\\.mysql\\.jdbc",
  "Zend_Db_(Adapter|Statement)_Mysqli_Exception",
  "Pdo[./_\\]Mysql",
  "MySqlException",
  "SQLSTATE\\[\\d+\\]: Syntax error or access violation",
  "check the manual that (corresponds to|fits) your MariaDB server version",
  "check the manual that (corresponds to|fits) your Drizzle server version",
  "MemSQL does not support this type of query",
  "is not supported by MemSQL",
  "unsupported nested scalar subselect",
  "PostgreSQL.*?ERROR",
  "Warning.*?\\Wpg_",
  "valid PostgreSQL result",
  "Npgsql\\.",
  "PG::SyntaxError:",
  "org\\.postgresql\\.util\\.PSQLException",
  "ERROR:\\s\\ssyntax error at or near",
  "ERROR: parser: parse error at or near",
  "PostgreSQL query failed",
  "org\\.postgresql\\.jdbc",
  "Pdo[./_\\]Pgsql",
  "PSQLException",
  "Driver.*? SQL[\\-\\_\\ ]*Server",
  "OLE DB.*? SQL Server",
  "\bSQL Server[^&lt;&quot;]+Driver",
  "Warning.*?\\W(mssql|sqlsrv)_",
  "\bSQL Server[^&lt;&quot;]+[0-9a-fA-F]{8}",
  "DB-Error.*",
  "System\\.Data\\.SqlClient\\.SqlException\\.(SqlException|SqlConnection\\.OnError)",
  "(?s)Exception.*?\bRoadhouse\\.Cms\\.",
  "Microsoft SQL Native Client error '[0-9a-fA-F]{8}",
  "\\[SQL Server\\]",
  "ODBC SQL Server Driver",
  "ODBC Driver \\d+ for SQL Server",
  "SQLServer JDBC Driver",
  "com\\.jnetdirect\\.jsql",
  "macromedia\\.jdbc\\.sqlserver",
  "Zend_Db_(Adapter|Statement)_Sqlsrv_Exception",
  "com\\.microsoft\\.sqlserver\\.jdbc",
  "Pdo[./_\\](Mssql|SqlSrv)",
  "SQL(Srv|Server)Exception",
  "Unclosed quotation mark after the character string",
  "Microsoft Access (\\d+ )?Driver",
  "JET Database Engine",
  "Access Database Engine",
  "ODBC Microsoft Access",
  "Syntax error \\(missing operator\\) in query expression",
  "\bORA-\\d{5}",
  "Oracle error",
  "Oracle.*?Driver",
  "Warning.*?\\W(oci|ora)_",
  "quoted string not properly terminated",
  "SQL command not properly ended",
  "macromedia\\.jdbc\\.oracle",
  "oracle\\.jdbc",
  "Zend_Db_(Adapter|Statement)_Oracle_Exception",
  "Pdo[./_\\](Oracle|OCI)",
  "OracleException",
  "CLI Driver.*?DB2",
  "DB2 SQL error",
  "\bdb2_\\w+\\(",
  "SQLCODE[=:\\d, -]+SQLSTATE",
  "com\\.ibm\\.db2\\.jcc",
  "Zend_Db_(Adapter|Statement)_Db2_Exception",
  "Pdo[./_\\]Ibm",
  "DB2Exception",
  "ibm_db_dbi\\.ProgrammingError",
  "Warning.*?\\Wifx_",
  "Exception.*?Informix",
  "Informix ODBC Driver",
  "ODBC Informix driver",
  "com\\.informix\\.jdbc",
  "weblogic\\.jdbc\\.informix",
  "Pdo[./_\\]Informix",
  "IfxException",
  "Dynamic SQL Error",
  "Warning.*?\\Wibase_",
  "org\\.firebirdsql\\.jdbc",
  "Pdo[./_\\]Firebird",
  "SQLite/JDBCDriver",
  "SQLite\\.Exception",
  "(Microsoft|System)\\.Data\\.SQLite\\.SQLiteException",
  "Warning.*?\\W(sqlite_|SQLite3::)",
  "\\[SQLITE_ERROR\\]",
  "SQLite error \\d+:",
  "sqlite3.OperationalError:",
  "SQLite3::SQLException",
  "org\\.sqlite\\.JDBC",
  "Pdo[./_\\]Sqlite",
  "SQLiteException",
  "SQL error.*?POS([0-9]+)",
  "Warning.*?\\Wmaxdb_",
  "DriverSapDB",
  "-3014.*?Invalid end of SQL statement",
  "com\\.sap\\.dbtech\\.jdbc",
  "\\[-3008\\].*?: Invalid keyword or missing delimiter",
  "Warning.*?\\Wsybase_",
  "Sybase message",
  "Sybase.*?Server message",
  "SybSQLException",
  "Sybase\\.Data\\.AseClient",
  "com\\.sybase\\.jdbc",
  "Warning.*?\\Wingres_",
  "Ingres SQLSTATE",
  "Ingres\\W.*?Driver",
  "com\\.ingres\\.gcf\\.jdbc",
  "Exception (condition )?\\d+\\. Transaction rollback",
  "com\\.frontbase\\.jdbc",
  "Syntax error 1. Missing",
  "(Semantic|Syntax) error [1-4]\\d{2}\\.",
  "Unexpected end of command in statement \\[",
  "Unexpected token.*?in statement \\[",
  "org\\.hsqldb\\.jdbc",
  "org\\.h2\\.jdbc",
  "\\[42000-192\\]",
  "![0-9]{5}![^]",
  "]+(failed|unexpected|error|syntax|expected|violation|exception)",
  "\\[MonetDB\\]\\[ODBC Driver",
  "nl\\.cwi\\.monetdb\\.jdbc",
  "Syntax error: Encountered",
  "org\\.apache\\.derby",
  "ERROR 42X01",
  ", Sqlstate: (3F|42).{3}, (Routine|Hint|Position):",
  "/vertica/Parser/scan",
  "com\\.vertica\\.jdbc",
  "org\\.jkiss\\.dbeaver\\.ext\\.vertica",
  "com\\.vertica\\.dsi\\.dataengine",
  "com\\.mckoi\\.JDBCDriver",
  "com\\.mckoi\\.database\\.jdbc",
  "&lt;REGEX_LITERAL&gt;",
  "com\\.facebook\\.presto\\.jdbc",
  "io\\.prestosql\\.jdbc",
  "com\\.simba\\.presto\\.jdbc",
  "UNION query has different number of fields: \\d+, \\d+",
  "Altibase\\.jdbc\\.driver",
  "com\\.mimer\\.jdbc",
  "Syntax error,[^",
  "]+assumed to mean",
  "io\\.crate\\.client\\.jdbc",
  "encountered after end of query",
  "A comparison operator is required here",
  "-10048: Syntax error",
  "rdmStmtPrepare\\(.+?\\) returned",
  "SQ074: Line \\d+:",
  "SR185: Undefined procedure",
  "SQ200: No table",
  "Virtuoso S0002 Error",
  "\\[(Virtuoso Driver|Virtuoso iODBC Driver)\\]\\[Virtuoso Server\\]"
}


payloads = {
    "'123",
    "''123",
    "`123",
    "\")123",
    "\"))123",
    "`)123",
    "`))123",
    "'))123",
    "')123\"123",
    "[]123",
    "\"\"123",
    "'\"123",
    "\"'123",
    "\123",
}

function scan(url,current_payload)
    resp = send_req(url)
    println(string.format("SQLI SENT %s",resp.status:GetStrOrNil()))
    if resp.body:GetStrOrNil() == "" then
        return 0
    end

    for index_key,index_value in ipairs(sqli_errors) do
        match = is_match(index_value,resp.body:GetStrOrNil()) 
        if match == false then
            -- NOTHING
        else
            report["url"] = url
            report["match"] = index_value
            report["payload"] = current_payload
            valid = true
            println("FOUND SQLI")
            return 1
        end
    end
    return 0
end

function main(url)
    stop = 0
    if string.find(url,"?") then
        for index_key, payload_value in ipairs(payloads) do
            new_querys = change_urlquery(url,payload_value)
            for url_index, new_url in pairs(new_querys) do 
                local out = scan(new_url, payload_value)
                if out == 1 then 
                    stop = 1
                    break
                end
            end
            if stop == 1 then
                break
            end
        end
    end
    return report
end

-- Run The Script
main(TARGET_URL)
