import initSqlJs from './sql-wasm.js';

let SQL;
initSqlJs().then(function (sql) {
    SQL = sql;
});

export class SqlJsDatabase {
    constructor() {
        this.db = new SQL.Database();
    }

    exec(sql) {
        this.db.run(sql);
    }
}