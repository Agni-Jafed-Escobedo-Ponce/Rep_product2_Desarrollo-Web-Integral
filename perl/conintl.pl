#!/usr/bin/perl
# perl/conintl.pl
# Ejecutar: plackup conintl.pl -p 5003
# URL: http://localhost:5003/?n=10
# Convierte un número a letras en español usando Lingua::ES::Numeros

use strict;
use warnings;
use Plack::Request;
use Lingua::ES::Numeros;

my $app = sub {
    my $env = shift;
    my $req = Plack::Request->new($env);
    
    my $n = $req->param('n') // 0;
    
    my $conv = Lingua::ES::Numeros->new(GENERO => 'M');
    my $texto = $conv->cardinal($n);
    
    return [
        200,
        ['Content-Type' => 'text/plain; charset=UTF-8'],
        [ $texto ]
    ];
};

# IMPORTANTE: Devolver la referencia de código
$app;