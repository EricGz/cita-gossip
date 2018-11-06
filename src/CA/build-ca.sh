#!/bin/sh

#this is the test version
#mainly focus on the function's working
#file path need to be adjusted, so some command should be extended

set -xe

#generate public and private key
#dir index:
#"CA-NODE": if this node is a CA node, then it must contains this dir, it stores the pub/private key for this node to authorized certs;also stores its index.txt
#"CA": each nodes has this dir, saves its .key and .cert

#clear file path
if test -d "CA-NODE"
then
    echo "CA-NODE exist"
    rm -r CA-NODE
else
    echo "no CA-NODE dir"
    mkdir CA-NODE
fi

if test -d "CA"
then
    echo "CA file exist"
    rm -r CA
else
    echo "no CA dir"
    mkdir CA
fi

#generate files
touch /CA-NODE/index.txt
echo 0001 > /CA-NODE/serial

#generate root keys
path1="CA"
path2="CA-NODE"
openssl genrsa 2048 -out $path1/ca.key
openssl req -new -key $path1/ca.key -out $path1/ca.csr
openssl x509 -req -days 3650 -in $path1/ca.csr -signkey $path1/ca.key -out $path1/ca.cert

#generate list of cert revoke
openssl ca -gencrl -out path/ca.crl -crldays 7

#move root cert to CA-NODE
cp $path1/ca.cert $path2

#generate un-CA node's key and csr(which is the cert request)





