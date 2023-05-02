import ws from 'ws';
import { neonConfig, Pool } from '@neondatabase/serverless';
neonConfig.webSocketConstructor = ws;
neonConfig.wsProxy = "localhost:8080";
neonConfig.useSecureWebSocket = false;

const pool = new Pool({ connectionString: process.env.DB })
pool.query('').then((x) => {
    console.log(x);
});
pool.query('1').then((x) => {
    console.log(x);
});
pool.query('12').then((x) => {
    console.log(x);
});
pool.query('123').then((x) => {
    console.log(x);
});
pool.query('1234567890').then((x) => {
    console.log(x);
});
