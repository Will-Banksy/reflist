# reflist

A small tool to generate a sorted, formatted reference list ready for pasting into a WYSIWSG editor, for those of us who haven't (yet) resorted to latex.

The tool takes a bibtex file (such as the one generated with [Citations](https://gitlab.gnome.org/World/citations)) and outputs the HTML that represents it,
in a format that can be understood by Google Docs and OnlyOffice (others will likely work).

On linux, the output can be piped through `xclip` or `wl-copy` to automatically copy it to the clipboard with the correct MIME type, e.g.:
```
reflist -i bibliography.bib -f 'Title: ${title}, etc.' | xclip -sel c -t "text/html"
```

This was 100% made to procrastinate actually doing any uni work and may well be reinventing the wheel (I am actually not sure but there's probably a few bibtex to html programs that exist) in which case you're probably better off using those.

## Usage

```
Usage: reflist [OPTIONS] --format-string <FORMAT_STRING>

Options:
  -i, --file <FILE>                    The bibtex (.bib) input file [default: -]
  -f, --format-string <FORMAT_STRING>  The format string, where fields are inserted at ${field_name}, e.g. the authors would be inserted at ${authors}
  -s, --sort-by <SORT_BY>              The field_name of the field to sort references by. Defaults to "author" [default: author]
  -h, --help                           Print help
  -V, --version                        Print version
```

## Format Strings

Specifying the format for reference output is done with format strings, which are just markdown but instances of ${field_name} are replaced by the value of field_name in the bibtex entry.

E.g. Here is a simple Harvard-esque format string for journal articles:
```
${AUTHOR} (${YEAR}) ‘${TITLE}’, *${JOURNAL}*, ${VOLUME}(${NUMBER}), pp. ${PAGES}. doi: ${DOI}
```
Example output:
```html
<ul>
<li>Ali, R.R. and Mohamad, K.M. (2021) ‘RX_myKarve carving framework for reassembling complex fragementations of JPEG images’, <span style="font-style:italic;">Journal of King Saud University. Computer and information sciences</span>, 33(1), pp. 21-32. doi: 10.1016/j.jksuci.2018.12.007</li>
<li>Pal, A. and Memon, N. (2009) ‘The evolution of file carving’, <span style="font-style:italic;">IEEE Signal Processing Magazine</span>, 26(2), pp. 59-71. doi: 10.1109/MSP.2008.931081</li>
</ul>
```

Currently, only one format string can be specified, and is used across all entry types. Obviously this isn't ideal, so this will probably be addressed at some point, if I continue developing this.