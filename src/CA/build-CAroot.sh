#!/bin/sh
#function: build CA root keys and certs
#generate for once and copy its key and cert to new node to sign their certs
#file path may need to be adjusted, so some command should be extended while practical using
set -xe
#dir index:
#"CA": each nodes has this dir, saves its .key and .cert
#clear file path and generate files
if test -d "CA"
then
    echo "CA file exist"
    rm -r CA
else
    echo "no CA dir"
    mkdir CA
fi
path='CA'
#generate root keys
#private key
openssl genrsa 2048 -out $path/ca.key
#public key
openssl rsa -in $path/ca.key -pubout -out $path/ca.pub
#generate cert req
openssl req -new -key $path/ca.key -out $path/ca.csr
#sign the cert by CA-self
openssl x509 -req -days 3650 -in $path/ca.csr -signkey $path/ca.key -out $path/ca.cert

#ca.key private key
#ca.pub public key
#ca.csr cert request
#ca.cert certificate




