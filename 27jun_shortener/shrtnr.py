import cherrypy, hashlib, datetime
from views import *

class URLDatabase:
    def __init__(self):
        self.db = {}

    def insert(self, url_id, actual):
        if url_id in self.db:
            return None # don't just overwrite the old one
        else:
            self.db[url_id] = {'url': actual, 'count': 0}
            return True

    def inc_count(self, url_id):
        self.db[url_id]['count'] += 1

    def lookup(self, url_id):
        if url_id in self.db:
            return self.db[url_id]['url']
        else:
            return None

    def visit(self, url_id):
        self.inc_count(url_id)
        return self.lookup(url_id)


# input is 
#  - a url to shorten
#  - a length the identifier should be
#  - another string (to be concatted with the URL) to attempt to make the hash unique
#    even if URL is submitted multiple times
def shorten_url(url, length, s):
    urlb = bytes(url+s, 'utf-8')
    digest = hashlib.sha1(urlb).hexdigest()
    return digest[:length]

def shorten(url):
    return shorten_url(url, 8, str(datetime.datetime.now()))


# currently we just strip the 'http://' out to normalize URLs
def preprocess_url(url):
    if url[:7] == "http://":
        return url[7:]
    else:
        return url


class Shrtnr(object):
    def __init__(self, db):
        self.db = db

    @cherrypy.expose
    def index(self):
        return render_index()

    @cherrypy.expose
    def shorten(self, url):
        preprocess = preprocess_url(url)
        url_id = shorten(preprocess)
        result = db.insert(url_id, preprocess)

        # it's possible we got a conflict. by design, shorten() should return something ne
        # each time, so keep attempting it until we succeed
        while result is None:
            url_id = shorten(url)
            result = db.insert(url_id, url)

        new_url = "/visit/"+url_id
        msg = "Here's yr shiny new URL: <a href='{0}'>{0}</a>".format(new_url)

        return render_page(msg)


    @cherrypy.expose
    def visit(self, url_id):
        msg = "Redirecting..."
        url = self.db.visit(url_id)
        return redirect_page(url)


    @cherrypy.expose
    def stats(self):
        entries = ""
        for k,v in self.db.db.items():
            entries += render_stats_table_entry(k, v['url'], v['count'])

        return render_page(render_stats_table(entries))


if __name__ == '__main__':
    db = URLDatabase()
    cherrypy.quickstart(Shrtnr(db))
