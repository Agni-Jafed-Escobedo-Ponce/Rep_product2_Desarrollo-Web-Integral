# ruby/clisoap1.rb
# Ejecutar: ruby clisoap1.rb
# URL: http://localhost:4567/?n=10
# Consume el servicio SOAP público NumberConversion y devuelve el número en letras (inglés)

require 'sinatra'
require 'savon'

set :port, 4567
set :bind, '0.0.0.0'

get '/' do
  n = params[:n] || '10'
  
  wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL'
  cliente = Savon.client(wsdl: wsdl, log: false)
  respuesta = cliente.call(:number_to_words, message: { ubi_num: n.to_i })
  resultado = respuesta.body[:number_to_words_response][:number_to_words_result]
  
  resultado.to_s
end