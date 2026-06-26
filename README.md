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

## Repository Layout

This repository separates translation text, shared styling, local styling, templates and generated output.

Source files:
- `files/ENGLISH/language.json`: canonical English text and metadata
- `files/<LANGUAGE>/language.json`: translated text and metadata for a language
- `files/shared_style.json`: shared style baseline for the whole repository
- `files/<LANGUAGE>/style.json`: optional per-language style overrides
- `files/<LANGUAGE>/image.png`: language flag or icon
- `files/<LANGUAGE>/glossary.json`: optional automation glossary for translation runs
- `files/<LANGUAGE>/translation_cache.json`: optional automation cache for translation runs

Generated output:
- `build/<LANGUAGE>/language.json`
- `build/<LANGUAGE>/image.png`

Templates copied by scripts:
- `templates/style.json`
- `templates/glossary.json`

## Automated Translation Script

For a faster first draft, this repository also includes `auto_translate.py`.

What it does:
- Creates or reuses a temporary Python virtual environment
- Installs the required translation package (`deep-translator`) if it is missing
- Reads `files/ENGLISH/language.json`
- Creates or updates `files/<LANGUAGE>/language.json`
- Copies `files/ENGLISH/image.png` if the target image does not exist yet
- Ensures `files/<LANGUAGE>/style.json` exists
- Ensures `files/<LANGUAGE>/glossary.json` exists
- Reuses a per-language glossary and translation cache when available

### Requirements
- Python 3 with the `venv` module available
- Internet access while generating translations

### Example usage
```bash
python3 auto_translate.py GERMAN \
  --target-code de \
  --anglicized-name German \
  --native-name Deutsch \
  --authors "Your Name"
```

```bash
python3 auto_translate.py FRENCH \
  --target-code fr \
  --anglicized-name French \
  --native-name Français \
  --authors "Your Name"
```

Only rebuild the translation cache:
```bash
python3 auto_translate.py GERMAN --rebuild-cache-only
```

### Reusable glossary and cache

The script stores reusable automation files inside the target language folder:
- `files/<LANGUAGE>/glossary.json`
- `files/<LANGUAGE>/translation_cache.json`
- `files/<LANGUAGE>/style.json`

#### `glossary.json`
This file is for manual control over wording.

It supports:
- `by_key`: exact override by internal entry name
- `by_source_text`: exact override by original English text
- `post_replace`: simple find/replace pairs applied after translation

#### `translation_cache.json`
This file stores previously translated source strings so later runs can reuse them instead of retranslating identical English text.

That means:
- repeated strings become more consistent
- reruns are faster
- you can improve a pack over time without losing prior work
- on each script run, the cache is refreshed from the current target `language.json` unless you use `--no-reuse-existing`

### Helpful options
- `--force-retranslate`: ignore cached translations and translate again
- `--no-reuse-existing`: do not seed the cache from an existing target `language.json`
- `--glossary-path PATH`: use a different glossary file
- `--cache-path PATH`: use a different cache file
- `--write-glossary-template-only`: create the glossary file and stop
- `--rebuild-cache-only`: rebuild `translation_cache.json` from the current target `language.json` and stop

### Notes
- The script is best used for creating a first draft, not as a final replacement for manual review.
- Rich-text tags, placeholders like `%s`, and keybind tokens like `${DriveShoot}` are (/should be) preserved automatically.

## Shared Style Pipeline

To separate translated text from presentation styling, this repository uses:
- a shared style baseline in `files/shared_style.json`
- optional per-language overrides in `files/<LANGUAGE>/style.json`

This setup exists because:
- most style fields are usually identical across languages
- some languages still need one-off overrides for size, alignment, or emphasis
- keeping shared style separate reduces duplication and keeps `language.json` focused on text

### Build merged output
Use `build_language_pack.py` to generate a final Unity/proceliotool-ready pack:

```bash
python3 build_language_pack.py GERMAN
```

This writes:
- `build/GERMAN/language.json`
- `build/GERMAN/image.png`

If you omit the language argument, the script builds every language folder found in `files/`:

```bash
python3 build_language_pack.py
```

Merge order:
1. canonical entry list from `files/ENGLISH/language.json`
2. translated values from `files/<LANGUAGE>/language.json`, falling back to English where entries are missing
3. shared style from `files/shared_style.json`
4. per-language overrides from `files/<LANGUAGE>/style.json`

### Recommendation
- Keep translated wording in `files/<LANGUAGE>/language.json`
- Put only true language-specific presentation changes into `files/<LANGUAGE>/style.json`
- Treat `build/<LANGUAGE>/language.json` as generated output, not as the place to edit translations manually

#### Deployment

For mass-release, lang files must be built and deployed serverside by one of the devs.

For local testing, your built .lang file can be put in the `localization` subfolder in Unity's PersistentDataPath. (`C:\Users\brenn\AppData\LocalLow\Procul Games\Procelio\localization\English.lang`, for example, on Windows). If the folder doesn't exist, you can create it yourself. Run Procelio, and the file should be visible in Settings -> Game Settings -> Language

## Customizing Translation Files

Each language pack consists of multiple files:
1. `language.json`: text and language metadata
2. `style.json`: optional style overrides for this language only
3. `image.png`: the image shown next to the language in the menu
4. `glossary.json`: optional automation glossary
5. `translation_cache.json`: optional automation cache

There is also one shared repository-wide style file:
- `files/shared_style.json`

And one generated output location:
- `build/<LANGUAGE>/language.json`

Note: "logbook" entries are provided by default but are considered optional as some can be very long. Feel free to simply delete any entries that you don't want to fill out.

#### Metadata

##### anglicized_name
The name of this language in ASCII English. "Spanish", not "Español"

##### native_name
The name of the language in that language. 

##### authors
You! Whatever attribution you want for putting in the work here (name, username, etc.)

##### version
The version of this translation file. Should be increased whenever a new release is ready

### The Text File: language.json
`language.json` is a text-first file.

It contains:
- language metadata
- a `language_elements` array
- per entry, only `name` and `value`

Example entry:
```
{
  "name": "SOME_TEXT_FIELD",
  "value": "Some Text Value In Game"
}
```

##### name
The internal name of the field, used so the game can tell which UI element this goes with.
DO NOT CHANGE

##### value
The text of the UI element. You almost certainly want to change this. If empty, falls back to default English value

### The Style File: style.json
`style.json` contains only language-specific style overrides keyed by entry name.

Example:
```json
{
  "meta": {
    "description": "Optional style overrides applied on top of files/shared_style.json"
  },
  "by_key": {
    "SOME_TEXT_FIELD": {
      "size": 28,
      "bold": true,
      "alignment": 5
    }
  }
}
```

Supported style properties:

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

### The Shared Style File: shared_style.json
`files/shared_style.json` contains the shared baseline style for the whole repository.

Languages should only override this in their local `style.json` when necessary.

### The Generated Build File
`build/<LANGUAGE>/language.json` is the generated final output used for Unity/proceliotool workflows.

It contains:
- the full canonical English key set
- translated values where available
- English fallback values where a language is missing keys
- shared style from `files/shared_style.json`
- local style overrides from `files/<LANGUAGE>/style.json`

### The Image File: image.png
A 48x24 pixel PNG image.
Use any image editor you want to modify it, but keep the file name the same.



## License
This project is made available under the Apache License 2.0.

## Contributions
Contributions are welcome, subject to the license above.
For mildly tech-savvy translators able to use git, feel free to submit PRs.
Otherwise, feel free to download this repository (the green "Clone/Download" button), fill out a translation locally, and send the json+png to one of the devs.

Exact translations aren't required: if rewording a description makes the translation more idiomatic in the target language, feel free to do so.
