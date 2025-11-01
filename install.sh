#!/bin/bash

set -euo pipefail

curl -LO https://github.com/nuuenkdaniel/invoice_constructor/releases/download/v1.0-alpha/invoice_constructor-1.1-linux.tar.gz
tar -xzf invoice_constructor-1.1-linux.tar.gz

sudo chmod +x invoice_constructor
sudo mv invoice_constructor /usr/local/bin/
mkdir ~/.local/share/invoice_constructor

mv examples templates ~/.local/share/invoice_constructor
