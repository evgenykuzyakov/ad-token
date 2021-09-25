#!/bin/bash

near call adtoken.near --accountId=adtoken.near storage_deposit '{"account_id": "'$1'"}' --amount=0.00109
near call adtoken.near --accountId=adtoken.near ft_transfer '{"receiver_id": "'$1'", "amount": "1"}' --amount=0.000000000000000000000001
