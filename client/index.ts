import ws from 'ws';
import { neonConfig, Pool } from '@neondatabase/serverless';
neonConfig.webSocketConstructor = ws;
neonConfig.wsProxy = "localhost:8080";
neonConfig.useSecureWebSocket = false;

const pool = new Pool({ connectionString: process.env.DB })
pool.query('SELECT now()').then((x) => {
    console.log(x);
});
pool.query('sLOvtselzWwyMbBYfg8Ez12mzLFT9DzlNGrRYzRqoKe6y92zn5Pvb9r1n2tUxvhB1ZQbXHB6igncaNfoMbB12Nigm22Vpg7WXZhxQLX82Wlk4kWKobZPtfCsyDPvKeM6RFn0AVb5r8CDI0KfTmjwlxhaPF8dtNjka6IFBKU8UkeRer9Tmjzm7P3VK2LUzJqbwDL8gLvpq6QQHsaNFrJMBM0BRImqvrNjgqchON1iiMTCvl3XqqrdWUtv5m').then((x) => {
    console.log(x);
});
pool.query('sLOvtselzWwyMbBYfg8Ez12mzLFT9DzlNGrRYzRqoKe6y92zn5Pvb9r1n2tUxvhB1ZQbXHB6igncaNfoMbB12Nigm22Vpg7WXZhxQLX82Wlk4kWKobZPtfCsyDPvKeM6RFn0AVb5r8CDI0KfTmjwlxhaPF8dtNjka6IFBKU8UkeRer9Tmjzm7P3VK2LUzJqbwDL8gLvpq6QQHsaNFrJMBM0BRImqvrNjgqchON1iiMTCvl3XqqrdWUtv5m123456').then((x) => {
    console.log(x);
});
