// nodejs/conintl.js
// Uso: node conintl.js  -> http://localhost:3003/?n=10
// Convierte un número a letras en español usando written-number

const http = require('http');
const url = require('url');
const writtenNumber = require('written-number');

const server = http.createServer((req, res) => {
  const { query } = url.parse(req.url, true);
  const n = parseInt(query.n) || 0;

  try {
    // Convertir número a letras en español
    const texto = writtenNumber(n, { lang: 'es' });

    res.writeHead(200, { 'Content-Type': 'text/plain; charset=utf-8' });
    res.end(texto);
  } catch (err) {
    console.error('Error:', err);
    res.writeHead(500);
    res.end(`Error: ${err.message}`);
  }
});

server.listen(3003, () => console.log('http://localhost:3003/?n=10'));