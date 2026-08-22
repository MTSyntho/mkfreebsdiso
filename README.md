<div align="center">
	<h1>mkfreebsdiso</h1>
	<p>Generate FreeBSD Images without source or a FreeBSD Host</p>
</div>

mkfreebsdiso is a simple CLI utility I've written which allows you to create FreeBSD derivatives
without the hard requirement of a FreeBSD host or building the entire OS from scratch.

It works by taking `base.txz` and `kernel.txz`, extracting them ( while preserving permissions and such ), overlaying your changes, 
then simply repackaging the filesystem contents into a bootable image.

I wrote this small tool for my other projects, I'm not the best at rust so I did my best...

I hope you find this tool useful though! <3

## 💻 Program Dependencies:
```
curl
xorriso
```
Please ensure you have these 2 packages installed on your machine in order for the tool to run!
`curl` is needed for downloading a fresh base system, `xorriso` is needed for assembling the final image.

## 🧰 How to use
To start off, you need to create a project directory, where we will define everything.
```
Usage: mkfreebsdiso project [OPTIONS]

Options:
  -m, --mode <MODE>            Specify how you wish to make modifications to the base system [default: direct]
  -v, --bsd-version <VERSION>  Specify a FreeBSD version to download from servers [default: 0.0]
  -b, --base-file <FILE>       Provide a FreeBSD base.txz file [default: base.txz]
  -k, --kernel-file <FILE>     Provide a FreeBSD kernel.txz file [default: kernel.txz]
  -p, --path <WORKDIR>         Specify working directory for the project [default: .]
  -h, --help                   Print help
```
If you do not already have a `base.txz` and `kernel.txz` file ready, you can pass `-v` ( or `--bsd-version` ) and a version number for the program 
to download for you via `curl`.
If you already have both files prepared, you can pass them in using `-b` ( or `--base-file` ) and `-k` ( or `--kernel-file` ) for the tool to copy into 
the new project directory which'll be generated afterwards.
`-m` ( or `--mode` ) is supposed to allow you to either overlay changes or make direct changes to the filesystem, however that functionality has yet to be implemented.

By default, an overlay approach is used.

☆ - ☆ - ☆

When it comes to building the final image, it is pretty straightforward!
**_NOTE: Root is required for this subcommand_**
```
Usage: mkfreebsdiso build [OPTIONS]

Options:
  -t, --type <FORMATS>  Specify the format(s) in which you wish to build the final image [default: iso]
  -o, --out <OUT>       Specify output directory of the resulting ISO image ( Split using commas, no spaces in-between ) [default: .]
  -h, --help            Print help
```
The supported file types are `iso`, `img`, and `txz`, however `img` requires a FreeBSD host due to requiring their UFS2/3 filesystem tooling.

At the moment, the only supported file format is `iso`, the others may come later down the line. The focus of the tool is purely on generating `iso` files anyway...

_NOTE: The `-o` ( or `--out` ) flag may or may not work... I don't recall implementing the logic to place the image in your desired output directory. My apologies for that!_

☆ - ☆ - ☆
