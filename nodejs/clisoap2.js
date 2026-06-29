// nodejs/clisoap2.js
// Uso: node clisoap2.js  -> http://localhost:3002/?n=10
// Consume el servicio SOAP y traduce el resultado al español

const http = require('http');
const url = require('url');
const soap = require('soap');
const { translate } = require('@vitalets/google-translate-api');

const WSDL = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';

const server = http.createServer(async (req, res) => {
  const { query } = url.parse(req.url, true);
  const n = parseInt(query.n) || 0;

  try {
    // 1. Llamada SOAP
    const cliente = await soap.createClientAsync(WSDL);
    const [respuesta] = await cliente.NumberToWordsAsync({ ubiNum: n });
    const en = respuesta.NumberToWordsResult;

    // 2. Traducción inglés -> español
    const result = await translate(en, { from: 'en', to: 'es' });
    const es = result.text;

    res.writeHead(200, { 'Content-Type': 'text/plain; charset=utf-8' });
    res.end(es);
  } catch (err) {
    console.error('Error:', err);
    res.writeHead(500);
    res.end(`Error: ${err.message}`);
  }
});

server.listen(3002, () => console.log('http://localhost:3002/?n=10'));