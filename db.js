// db.js
import Database from "better-sqlite3";
import fs from "fs";
import path from "path";

const DB_FILE = process.env.SQLITE_FILE || "data/app.db";
const MIGRATIONS_DIR = path.join(new URL(import.meta.url).pathname, "migrations");

// garante diretório
const dbDir = path.dirname(DB_FILE);
if (!fs.existsSync(dbDir) && dbDir !== ".") fs.mkdirSync(dbDir, { recursive: true });

// abre banco
const db = new Database(DB_FILE);

// função utilitária para executar uma migration SQL (transaction)
function applyMigration(name, sql) {
  console.log(`Aplicando migration: ${name}`);
  const tran = db.transaction(() => {
    db.exec(sql);
    db.prepare(
      `INSERT INTO migrations_applied (name, applied_at) VALUES (?, datetime('now'))`
    ).run(name);
  });
  tran();
}

// cria tabelas de controle se não existirem
db.exec(`
  CREATE TABLE IF NOT EXISTS migrations_applied (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    applied_at TEXT NOT NULL
  );
`);

// lê todas as migrations (arquivos .sql na pasta migrate)
function runPendingMigrations() {
  const migrationsPath = path.join(process.cwd(), "migrations");
  if (!fs.existsSync(migrationsPath)) {
    console.log("Pasta migrations não existe. Criando pasta migrations/...");
    fs.mkdirSync(migrationsPath);
    return;
  }

  const files = fs.readdirSync(migrationsPath).filter(f => f.endsWith(".sql")).sort();
  const applied = db.prepare("SELECT name FROM migrations_applied").all().map(r => r.name);

  for (const file of files) {
    if (applied.includes(file)) {
      // já aplicada
      continue;
    }
    const sql = fs.readFileSync(path.join(migrationsPath, file), "utf8");
    applyMigration(file, sql);
  }
}

// útil para debugging
function query(sql, params = []) {
  return db.prepare(sql).all(...params);
}

if (process.argv[1].endsWith("db.js")) {
  // rodando node db.js -> aplicar migrations
  runPendingMigrations();
  console.log("Migrations aplicadas (se houver).");
  process.exit(0);
}

export { db, runPendingMigrations, query };
