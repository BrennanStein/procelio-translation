# Procelio Translation Tool + Files

The executable is used for generating and compiling translation files. It *must* be ran from the command line.

## Usage

#### `proceliotool.exe lang .\path\to\where\to\save\MyLanguageName (.\path\to\entries.txt)`
- If MyLanguageName doesn't exist, creates a new localization file
- If MyLanguageName does exist _but_ doesn't contain all localization entities as entries.txt, updates the file to contain all
- If MyLanguageName does exist and is up-to-date, compiles it to AnglicizedName.lang

#### A sample run
```
# Acquire files and open CMD in the directory. Via CLI, or download/extract the repo from github
# If tech-savvy, you can do a fork and submit a PR back up when you're done
git clone https://github.com/BrennanStein/procelio-translation.git
cd procelio-translation

# Actually run the tool
.\proceliotool.exe lang .\files\MyNewLanguage  # Will generate image+data files. Go edit them
.\proceliotool.exe lang .\files\MyNewLanguage  # Will generate '.lang' file
```

(For creating a new file, you can also copy-paste the English pack and overwrite data as necessary)

#### Deployment

For mass-release, lang files must be built and deployed serverside by one of the devs.

For local testing, your built .lang file can be put in the `localization` subfolder in Unity's PersistentDataPath. (`C:\Users\brenn\AppData\LocalLow\Procul Games\Procelio\localization\English.lang`, for example, on Windows). If the folder doesn't exist, you can create it yourself. Run Procelio, and the file should be visible in Settings -> Game Settings -> Language

## Customizing Translation Files
Each translation file consists of two pieces:
1) The JSON file, where all of the data is defined for the translation
2) The PNG Image file, which should appear next to the native name in the translation menu (expected to be a country flag)

### The JSON file: language.json
The "language.json" file holds all of the translation data for this translation.
This file contains a bit of metadata, then a large list of language elements that define the actual user interface text.

#### Metadata

##### anglicized_name
The name of this language in ASCII English. "Spanish", not "Español"

##### native_name
The name of the language in that language. 

##### authors
You! Whatever attribution you want for putting in the work here (name, username, etc.)

##### version
The version of this translation file. Should be increased whenever a new release is ready

#### Language Elements
For saving space, only the field_name and field_value values are generated by default.
If further alternations are required, various parameters of the text can be modified by adding the corresponding fields.

Here is a fully-specified text element
```
{
      "name": "SOME_TEXT_FIELD",
      "value": "Some Text Value In Game",
      "size": 0,
      "bold": false,
      "italic": false,
      "underline": false,
      "strikethrough": false,
      "alignment": 0,
      "color": [
        255,
        255,
        255
      ]
}
```

Descriptions of each field are as follows:

##### name
The internal name of the field, used so the game can tell which UI element this goes with.
DO NOT CHANGE

##### value
The text of the UI element. You almost certainly want to change this. If empty, falls back to default English value

##### size
In case it's necessary to overwrite the size of the text. 0 = default. 1024 max value. Must be positive.
If unspecified, treated as 0 (i.e. don't override)

##### bold
Whether the field value should be bold in-game
If unspecified, treated as 'false'

##### italic
Whether the field value should be italic in-game
If unspecified, treated as 'false'

##### underline
Whether the field value should be underlined in game
If unspecified, treated as 'false'

##### strikethrough
Whether the field value should be struck through in-game
If unspecified, treated as 'false'

##### alignment
The alignment to use for text. If unspecified, treated as 0 (i.e. don't override)
```
1 -> TextAlignmentOptions.TopLeft
2 -> TextAlignmentOptions.Top
3 -> TextAlignmentOptions.TopRight
4 -> TextAlignmentOptions.Left
5 -> TextAlignmentOptions.Center
6 -> TextAlignmentOptions.Right
7 -> TextAlignmentOptions.BottomLeft
8 -> TextAlignmentOptions.Bottom
9 -> TextAlignmentOptions.BottomRight
10 -> TextAlignmentOptions.TopJustified
11 -> TextAlignmentOptions.Justified
12 -> TextAlignmentOptions.BottomJustified
13 -> TextAlignmentOptions.BaselineLeft
14 -> TextAlignmentOptions.Baseline
15 -> TextAlignmentOptions.BaselineRight
```

##### color
What color the text should be in-game
If unspecified, treated as white '[255, 255, 255]'. Each number is the respective one-byte R,G,B channel

### The Image File: image.png
A 24x48 pixel PNG image.
Use any image editor you want to modify it, but keep the file name the same.



## License
This project is made available under the Apache License 2.0.

## Contributions
Contributions are welcome, subject to the license above.
For mildly tech-savvy translators able to use git, feel free to submit PRs.
Otherwise, feel free to download this repository (the green "Clone/Download" button), fill out a translation locally, and send the json+png to one of the devs.

Exact translations aren't required: if rewording a description makes the translation more idiomatic in the target language, feel free to do so.
