PATCH="$1"
echo "Downloading patch"
wget https://ddragon.leagueoflegends.com/cdn/dragontail-$PATCH.tgz
echo "Extracting patch"
tar xzf dragontail-$PATCH.tgz
echo "Applying fixes"
mv $PATCH/img/champion/Fiddlesticks.png $PATCH/img/champion/FiddleSticks.png
cp 0.png $PATCH/img/item/
echo "update patches.json!"