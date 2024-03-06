TODO: Testing

# reflist

A small tool to generate a sorted, formatted reference list ready for pasting into a WYSIWSG editor, for those of us who haven't (yet) resorted to latex.

The tool takes a bibtex file (such as the one generated with [Citations](https://gitlab.gnome.org/World/citations) (although you may want to modify it to
show the entry types and fields you need)) and outputs the HTML that represents it, in a format that can be understood by Google Docs and OnlyOffice (others
will likely work).

To get started with reflist, you need to write a TOML config file containing format strings for each entry type you want (see the section on format strings
for more on them). An example of this config file is included: [Reflist.toml](Reflist.toml). The config file also allows you to specify any styling that should be done
on the output text, for the case of editors like OnlyOffice which don't automatically apply the default style to italic/bold/etc. text.

~~On linux, the output can be piped through `xclip` or `wl-copy` to automatically copy it to the clipboard with the correct MIME type~~ Reflist now supports automatic copying to clipboard in HTML format (with a plain text backup, which for now is in markdown format) with the `-c` option.

This was 100% made to procrastinate actually doing any uni work and may well be reinventing the wheel (I am actually not sure but there's probably a few bibtex to html programs that exist) in which case you're probably better off using those.

## Usage

```
Usage: reflist [OPTIONS]

Options:
  -i, --file <FILE>                The bibtex (.bib) input file [default: -]
  -f, --config-file <CONFIG_FILE>  The path to the config file [default: Reflist.toml]
  -s, --sort-by <SORT_BY>          The field_name of the field to sort references by [default: author]
  -c, --copy                       Whether to automatically copy the html to clipboard, in html format (ready for pasting into a WYSIWYG editor)
  -h, --help                       Print help
  -V, --version                    Print version
```

A typical run of Reflist looks like:
```
reflist -i bibliography.bib -c
```

## Format Strings

Specifying the format for reference output is done with format strings, which are just markdown but instances of ${field_name} are replaced by the value of field_name in the bibtex entry.

E.g. Here is a simple Harvard-esque format string for journal articles (the format my uni uses):
```
${author} (${year}) ‘${title}’, *${journal}*, ${volume}(${number}), pp. ${pages}. doi: ${doi}
```
Example output:
```html
<ul>
<li>Ali, R.R. and Mohamad, K.M. (2021) ‘RX_myKarve carving framework for reassembling complex fragementations of JPEG images’, <span style="font-style:italic;">Journal of King Saud University. Computer and information sciences</span>, 33(1), pp. 21-32. doi: 10.1016/j.jksuci.2018.12.007</li>
<li>Pal, A. and Memon, N. (2009) ‘The evolution of file carving’, <span style="font-style:italic;">IEEE Signal Processing Magazine</span>, 26(2), pp. 59-71. doi: 10.1109/MSP.2008.931081</li>
</ul>
```

> - Ali, R.R. and Mohamad, K.M. (2021) ‘RX_myKarve carving framework for reassembling complex fragementations of JPEG images’, *Journal of King Saud University. Computer and information sciences*, 33(1), pp. 21-32. doi: 10.1016/j.jksuci.2018.12.007
> - Pal, A. and Memon, N. (2009) ‘The evolution of file carving’, *IEEE Signal Processing Magazine*, 26(2), pp. 59-71. doi: 10.1109/MSP.2008.931081
