def page_header():
    return """<html>
    <head><title>shrtnr</title>
    <style type="text/css">
    table,th,td { border: 1px solid black; }
    </style></head>
    <body><div style="border-bottom: 3px dotted red; margin-bottom: 20px"><a href="/">shrtnr</a></div>"""

def page_footer():
    return "</body></html>"

def render_page(content):
    return page_header() + content + page_footer()


def redirect_page(url):
    return """<html>
    <head><script type="text/javascript">window.location.replace("http://{}")</script></head></html>""".format(url)

def render_stats_table(entries):
        return """<table>
                    <tr>
                        <th>ID</th>
                        <th>URL</th>
                        <th>count</th>
                    </tr>
                    {}
                    </table>""".format(entries)


def render_stats_table_entry(url_id, url, count):
            return """<tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td></tr>""".format(url_id, url, count)


def render_index():
        return render_page("""
        <form method="post" action="shorten">
            <input type="text" name="url" placeholder="Type a URL..." />
            <button type="submit">Shrtn it</button>
        </form>""")
