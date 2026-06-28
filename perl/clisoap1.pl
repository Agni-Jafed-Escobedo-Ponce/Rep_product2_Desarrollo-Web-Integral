#!/usr/bin/perl
# perl/clisoap1.pl
# Ejecutar: plackup clisoap1.pl -p 5001
# URL: http://localhost:5001/?n=10
# Consume el servicio SOAP público NumberConversion y retorna el número en letras (inglés)

use strict;
use warnings;
use SOAP::Lite;
use Plack::Request;

my $app = sub {
    my $env = shift;
    my $req = Plack::Request->new($env);
    
    my $n = $req->param('n') // 10;
    
    my $wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
    my $cliente = SOAP::Lite->service($wsdl);
    my $resultado = $cliente->NumberToWords($n);
    
    return [
        200,
        ['Content-Type' => 'text/plain; charset=UTF-8'],
        [ $resultado ]
    ];
};

# IMPORTANTE: Devolver la referencia de código para plackup
$app;