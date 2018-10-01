module Main exposing (Model, Msg(..), CoinPair, config, init, main, coin, update, view)

import Browser
import Html exposing (Html, div, h1, input, text, button)
import Html.Attributes exposing (placeholder)
import Html.Events exposing (onInput, onClick)
import Table

import Http
import Json.Decode as Decode

import Debug

-- json

type alias CoinPair = { pair: String, price : Float }

getCoinList : Cmd Msg
getCoinList =
    let
        url = "prices.json"
        request = Http.get url decodeCoinList
    in
        Http.send GotCoinPair request

decodeCoinList : Decode.Decoder (List CoinPair)
decodeCoinList =
    Decode.list decodeCoinPair

decodeCoinPair : Decode.Decoder CoinPair
decodeCoinPair =
  Decode.map2 CoinPair
    (Decode.field "pair" Decode.string)
    (Decode.field "price" Decode.float)

-- main

main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }

-- MODEL

type alias Model =
    { coins : List CoinPair
    , tableState : Table.State
    , query : String
    }

init : () -> ( Model, Cmd Msg )
init flags =
    let
        model =
            {
                coins = [],
                tableState = Table.initialSort "Pair",
                query = ""
            }
    in
    ( model, Cmd.none )

--init : List CoinPair -> ( Model, Cmd Msg )
--init coins =
--    let
--        model =
--            {
--                coins = coins,
--                tableState = Table.initialSort "Pair",
--                query = ""
--            }
--    in
--    ( model, Cmd.none )

-- UPDATE

type Msg
    = SetQuery String
    | SetTableState Table.State
    | GetCoinPair
    | GotCoinPair (Result Http.Error (List CoinPair))

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetQuery newQuery ->
            ( { model | query = newQuery }, Cmd.none )

        SetTableState newState ->
            ( { model | tableState = newState }, Cmd.none)

        GetCoinPair ->
            let
                coins = getCoinList
                _ = Debug.log "tester" coins
            in
                (model, coins)

        GotCoinPair result ->
            case result of
                Err e ->
                    let
                        _ = Debug.log "Error in GotCoinPair: " e
                    in
                        (model, Cmd.none)
                Ok newCoins ->
                    let
                        _ = Debug.log "GotCoinPair" newCoins
                    in
                        ( { model | coins = newCoins }, Cmd.none)

-- VIEW

view : Model -> Html Msg
view { coins, tableState, query } =
    let
        lowerQuery =
            String.toLower query

        acceptablePeople =
            List.filter (String.contains lowerQuery << String.toLower << .pair) coins
    in
    div []
        [
            input [ placeholder "Search by Name", onInput SetQuery ] [],
            Table.view config tableState acceptablePeople,
            div [][ button [ onClick GetCoinPair ] [ text "Get Symbols" ]]
        ]

config : Table.Config CoinPair Msg
config =
    Table.config
        { toId = .pair
        , toMsg = SetTableState
        , columns =
            [ Table.stringColumn "Pair" .pair
            , Table.floatColumn "Price" .price
            ]
        }

-- PEOPLE

coin : List CoinPair
coin =
    [
        CoinPair "ADABTC" 40,
        CoinPair "NULSBTC" 400
    ]
