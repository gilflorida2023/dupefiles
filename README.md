
# dupefiles

Finds all duplicate files in a specified sub-directory tree specified on command-line and writes the results to a csv file.

Note: dupefiles skips hidden files/directories and zero byte files.

`$./dupefiles`
`Usage: ./dupefiles \<directory>`
`Finds all duplicate files in a specified sub-directory tree specified on command-line.`

`$./dupefiles /home/minty/Downloads/`
`DUPE1.NAME,DUPE1.SIZE,DUPE2.NAME,DUPE2.SIZEa`
`"/home/minty/Downloads/dupe.jpg",85448,"/home/minty/Downloads/Chung_1_1000.jpg",85448`
