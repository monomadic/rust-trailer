#!/bin/sh
cd server/frontend/
elm make src/App.elm --output="../../server/static/index.html"
