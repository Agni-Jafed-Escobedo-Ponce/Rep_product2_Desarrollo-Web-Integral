#!/usr/bin/perl
# perl/clisoap2.pl
# Ejecutar: plackup clisoap2.pl -p 5002
# URL: http://localhost:5002/?n=10
# Consume el servicio SOAP y traduce el resultado al español

use strict;
use warnings;
use SOAP::Lite;
use Plack::Request;
use LWP::UserAgent;
use JSON;
use URI;

# Función para traducir usando Google Translate API gratuita
sub google_translate {
    my ($texto, $origen, $destino) = @_;
    $origen //= 'en';
    $destino //= 'es';
    
    my $ua = LWP::UserAgent->new;
    my $uri = URI->new('https://translate.googleapis.com/translate_a/single');
    $uri->query_form(
        client => 'gtx',
        sl     => $origen,
        tl     => $destino,
        dt     => 't',
        q      => $texto
    );
    
    my $response = $ua->get($uri);
    
    if ($response->is_success) {
        my $json = decode_json($response->decoded_content);
        return join('', map { $_->[0] } @{$json->[0]});
    } else {
        return $texto;
    }
}

my $app = sub {
    my $env = shift;
    my $req = Plack::Request->new($env);
    
    my $n = $req->param('n') // 10;
    
    my $wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
    my $cliente = SOAP::Lite->service($wsdl);
    my $resultado_en = $cliente->NumberToWords($n);
    
    my $resultado_es = google_translate($resultado_en, 'en', 'es');
    
    return [
        200,
        ['Content-Type' => 'text/plain; charset=UTF-8'],
        [ $resultado_es ]
    ];
};

# IMPORTANTE: Devolver la referencia de código
$app;