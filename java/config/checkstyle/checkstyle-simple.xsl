<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0"
                xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
                xmlns:fo="http://www.w3.org/1999/XSL/Format">
  <xsl:output method="html" indent="yes" encoding="UTF-8"/>
  
  <xsl:template match="/">
    <html>
      <head>
        <title>Checkstyle Report</title>
        <style type="text/css">
          body { font-family: Arial, sans-serif; margin: 20px; }
          h1 { color: #333; }
          table { border-collapse: collapse; width: 100%; margin-top: 20px; }
          th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
          th { background-color: #f2f2f2; }
          tr:nth-child(even) { background-color: #f9f9f9; }
          .error { color: #d9534f; }
          .warning { color: #f0ad4e; }
          .info { color: #5bc0de; }
        </style>
      </head>
      <body>
        <h1>Checkstyle Report</h1>
        <table>
          <tr>
            <th>File</th>
            <th>Line</th>
            <th>Severity</th>
            <th>Message</th>
            <th>Rule</th>
          </tr>
          <xsl:for-each select="checkstyle/file">
            <xsl:variable name="filename" select="@name"/>
            <xsl:for-each select="error">
              <tr>
                <td><xsl:value-of select="$filename"/></td>
                <td><xsl:value-of select="@line"/></td>
                <td class="{@severity}">
                  <xsl:value-of select="@severity"/>
                </td>
                <td><xsl:value-of select="@message"/></td>
                <td><xsl:value-of select="@source"/></td>
              </tr>
            </xsl:for-each>
          </xsl:for-each>
        </table>
      </body>
    </html>
  </xsl:template>
</xsl:stylesheet>
