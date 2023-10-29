name=$1

if [ -f "./contest/$name.py" ]; then
    echo "$name exists."
else 
    touch "./contest/$name.py"
    touch "./contest/$name.txt"
fi

rm p.py
ln -s $(realpath ./contest/$name.py) ./p.py
rm input.txt
ln -s $(realpath ./contest/$name.txt) ./input.txt

