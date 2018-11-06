#!/bin/sh
#function: use CA key and cert to sign ordinary node's cert
#file path may need to be adjusted, so some command should be extended while practical using
set -xe
#dir index:
#"ca": saves its .key and .cert
#clear file path and generate files
if test -d "ca"
then
    echo "ca file exist"
    rm -r ca
else
    echo "no CA dir"
    mkdir ca
fi
CA_path='CA'
path='ca'
#generate root keys
#private key
openssl genrsa 2048 -out $path/node.key
#public key
openssl rsa -in $path/node.key -pubout -out $path/node.pub
#generate cert req
openssl req -new -key $path/node.key -out $path/node.csr
#sign the cert by CA-self
openssl ca -days 3650 -in $path/node.csr -cert $CA_path/ca.cert -keyfile $CA_path/ca.key -out $path/node.cert

#node.key private key
#node.pub public key
#node.csr cert request
#node.cert certificate





