# Kana/ASCII Conversion Utility 0.1.0

Kana conversion utilities to convert Japanese katakana and hiragana, along
with ASCII characters, into full or half-width forms.

There aer various requirements around storing data, reporting, user presentation, 
etc. with regard to Japanese scripts and ASCII characters.  With many services, 
data must be sent in a particular format (one example is Japanese CIC
credit reporting, where katakana must be sent in half-width, single-byte format,
while kanji and any alphabetical ASCII characters (i.e. "romaji") must be sent
as full-width, double-byte characters).  In other cases, data is stored as all
full-width, double-byte characters, awaiting transformation as required by 
separate utilities.  This library aims to help in these conversions, making
storing and sending Japanese characters easier.

# Usage

Simply add to your `Cargo.toml`:

```$xslt
[dependencies]
kana-conversion = "0.1"
```

and use in your rust code:

```$xslt
extern crate kana_conversion;
```

# Conversion Functions

The "to_double_byte" function takes a character to convert and a
mode.  If AsciiOnly is selected, only normal ASCII chars will be
converted to
