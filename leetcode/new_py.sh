# name=$1
name="p"

if [ -f "./problem/$name.py" ]; then
    echo "$name.py exists."
else 
    touch "./problem/$name.py"
    touch "./problem/$name.txt"
fi

rm p.py
ln -s $(realpath ./problem/$name.py) ./p.py
rm input.txt
ln -s $(realpath ./problem/$name.txt) ./input.txt
echo "python synced"

