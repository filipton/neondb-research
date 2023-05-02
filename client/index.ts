import ws from 'ws';
import fs from 'fs';
import { neonConfig, Pool } from '@neondatabase/serverless';
neonConfig.webSocketConstructor = ws;
neonConfig.wsProxy = "localhost:8080";
neonConfig.useSecureWebSocket = false;

const pool = new Pool({ connectionString: process.env.DB })
pool.query('SELECT now()').then((x) => {
    console.log(x);
});
/*
pool.query(fs.readFileSync('./data.txt').toString()).then((x) => {
    console.log(x);
});
*/
