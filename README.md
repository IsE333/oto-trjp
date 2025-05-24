# OTO-TRJP

A simple tool for generating japanese aliases for existing **oto.ini** using **dict.txt**. (Works with Shift-JIS encoded files only) 

## Usage

Run the exe file after setting the files below. 

1. **oto.ini**  
   – This file will be processed and the result is generated and added to **oto_new.ini**.  
   – Original lines remain **unchanged**, new lines are added to the end of file.  
2. **dict.txt**  
   – Lines must use this format: "kana romaji"  
3. **glottalstopletter.txt**  
   – Default character is set to ・
