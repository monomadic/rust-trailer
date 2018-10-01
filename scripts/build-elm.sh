#!/bin/sh
cd server/frontend/
elm make src/Prices.elm --output="server/static/index.html"
