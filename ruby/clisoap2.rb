# ruby/clisoap2.rb
# Ejecutar: ruby clisoap2.rb
# URL: http://localhost:4568/?n=10
# Consume el servicio SOAP público NumberConversion y traduce el resultado a español

require 'sinatra'
require 'savon'
require 'tr4n5l4te'

set :port, 4568
set :bind, '0.0.0.0'

get '/' do
  n = params[:n] || '10'
  
  # Consumir servicio SOAP
  wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL'
  cliente = Savon.client(wsdl: wsdl, log: false)
  respuesta = cliente.call(:number_to_words, message: { ubi_num: n.to_i })
  resultado_ingles = respuesta.body[:number_to_words_response][:number_to_words_result]
  
  # Traducir de inglés a español usando tr4n5l4te
  traductor = Tr4n5l4te::Translator.new
  resultado_espanol = traductor.translate(resultado_ingles, :en, :es)
  
  resultado_espanol
end