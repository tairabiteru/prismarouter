"""Main routing file."""

from py.conf import conf

from aiohttp import web
from aiohttp_jinja2 import template
from aiohttp_session import get_session
from subprocess import Popen


routes = web.RouteTableDef()

@routes.get("/")
#@template("index.html")
async def index_GET(request):
    """Handle GET requests for /"""
    request.app.prismarouter.terminate()
    request.app.prismarouter = Popen([request.app.prexec])
    return web.Response(text="YEP!")
