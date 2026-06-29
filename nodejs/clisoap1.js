// nodejs/clisoap1.js
// Uso: node clisoap1.js  -> http://localhost:3001/?n=10
// Consume el servicio SOAP público NumberConversion y retorna el número en letras (inglés)

const http = require('http');
const url = require('url');
const soap = require('soap');

const WSDL = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';

const server = http.createServer(async (req, res) => {
  const { query } = url.parse(req.url, true);
  const n = parseInt(query.n) || 0;

  try {
    const cliente = await soap.createClientAsync(WSDL);
    const [respuesta] = await cliente.NumberToWordsAsync({ ubiNum: n });
    const resultado = respuesta.NumberToWordsResult;

    res.writeHead(200, { 'Content-Type': 'text/plain; charset=utf-8' });
    res.end(resultado);
  } catch (err) {
    res.writeHead(500);
    res.end(`Error: ${err.message}`);
  }
});

server.listen(3001, () => console.log('http://localhost:3001/?n=10'));