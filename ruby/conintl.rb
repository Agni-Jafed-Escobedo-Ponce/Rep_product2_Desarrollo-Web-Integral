# ruby/conintl.rb
# Ejecutar: ruby conintl.rb
# URL: http://localhost:4569/?n=10
# Convierte un número a letras en español usando la gema 'humanize'

require 'sinatra'
require 'humanize'

set :port, 4569
set :bind, '0.0.0.0'

get '/' do
  n = params[:n] || '10'
  
  # Convertir número a palabras en español
  resultado = n.to_i.humanize(locale: :es)
  
  resultado
end