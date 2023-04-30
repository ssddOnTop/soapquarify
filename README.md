# About

This is project by team SSDD for HachNUThon (TechHolding).

This project stores and allows updating SOAP(xml) data and responds to various queries in form of JSON.

# Advantages

We recognised that at some period of time, there are either too many requests from individual clients or there are very
less amount of requests.

So we decided to use WebSockets over HTTP. We also cache queries for 2 minutes, so this prevents unnecessary sterilizing
and deserializing data.

# Usage

## Prerequisites:

Latest version of cargo (nightly) and some websocket client to use it.

## Usage

```
cargo run --package stj --bin stj -- -p <foo>/pw.txt -d <foo>
```

where pw.txt contains password and foo is path to the directory where xml will be stored.

### Example files:

| File          | Value                |
|---------------|----------------------|
| password file | [pw.txt](pw.txt)     |
| SOAP xml file | [file.xml](file.xml) |



### To insert values/update the xml file:

```json

{
  "id": "xml",
  "pw": "<password>",
  "value": "<xml>"
}
```

Example:

```json

{
  "id": "xml",
  "pw": "tmppw",
  "value": "<?xml version = \"1.0\"?>\n<SOAP-ENV:Envelope\n        xmlns:SOAP-ENV = \"http://www.w3.org/2001/12/soap-envelope\"\n        SOAP-ENV:encodingStyle = \"http://www.w3.org/2001/12/soap-encoding\">\n\n  <SOAP-ENV:Body xmlns:m = \"http://www.xyz.org/quotations\">\n    <m:GetQuotation>\n      <m:QuotationsName>MiscroSoft</m:QuotationsName>\n    </m:GetQuotation>\n  </SOAP-ENV:Body>\n</SOAP-ENV:Envelope>"
}
```

### To query value:

```json
{
  "id": "qry",
  "path": "path/to/value"
}
```

Example:

```json
{
  "id": "qry",
  "path": "Body/GetQuotation/QuotationsName/$value"
}
```

and to get full xml as JSON, just pass empty value in path i.e.:

```json
{
  "id": "qry",
  "path": ""
}
```

# Security

To prevent unauthorised people from updating the xml, we protected it by storing password in the linux/macos keyring.



