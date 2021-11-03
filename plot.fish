#!/usr/bin/fish

cd (dirname (status --current-filename))

if not test -d plots_env
    python3 -m venv plots_env
    source plots_env/bin/activate.fish
    pip install matplotlib pandas scipy
else
    source plots_env/bin/activate.fish
end

mkdir plots
mkdir data
cd data

for c in ../configs/*
    cargo run --release -- run -n 10000 -c $c --csv-write
end

mkdir plots

python3 ../plots.py

mv -f plots/* ../plots
rm plots -rf
cd ..

deactivate
