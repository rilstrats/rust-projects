"""
Course: CSE 251 
Lesson Week: 02
File: assignment.py 
Author: Brother Comeau

Purpose: Retrieve Star Wars details from a server

Instructions:

- Each API call must only retrieve one piece of information
- You are not allowed to use any other modules/packages except for the ones used
  in this assignment.
- Run the server.py program from a terminal/console program.  Simply type
  "python server.py"
- The only "fixed" or hard coded URL that you can use is TOP_API_URL.  Use this
  URL to retrieve other URLs that you can use to retrieve information from the
  server.
- You need to match the output outlined in the decription of the assignment.
  Note that the names are sorted.
- You are requied to use a threaded class (inherited from threading.Thread) for
  this assignment.  This object will make the API calls to the server. You can
  define your class within this Python file (ie., no need to have a seperate
  file for the class)
- Do not add any global variables except for the ones included in this program.

The call to TOP_API_URL will return the following Dictionary(JSON).  Do NOT have
this dictionary hard coded - use the API call to get this.  Then you can use
this dictionary to make other API calls for data.

{
   "people": "http://127.0.0.1:8790/people/", 
   "planets": "http://127.0.0.1:8790/planets/", 
   "films": "http://127.0.0.1:8790/films/",
   "species": "http://127.0.0.1:8790/species/", 
   "vehicles": "http://127.0.0.1:8790/vehicles/", 
   "starships": "http://127.0.0.1:8790/starships/"
}
{
    'title': 'Revenge of the Sith', 'episode_id': 3, 'opening_crawl': 'War! The Republic is crumbling\r\nunder attacks by the ruthless\r\nSith Lord, Count Dooku.\r\nThere are heroes on both sides.\r\nEvil is everywhere.\r\n\r\nIn a stunning move, the\r\nfiendish droid leader, General\r\nGrievous, has swept into the\r\nRepublic capital and kidnapped\r\nChancellor Palpatine, leader of\r\nthe Galactic Senate.\r\n\r\nAs the Separatist Droid Army\r\nattempts to flee the besieged\r\ncapital with their valuable\r\nhostage, two Jedi Knights lead a\r\ndesperate mission to rescue the\r\ncaptive Chancellor....', 'director': 'George Lucas', 'producer': 'Rick McCallum', 'release_date': '2005-05-19',
    'characters': ['http://127.0.0.1:8790/people/1/', 'http://127.0.0.1:8790/people/2/', 'http://127.0.0.1:8790/people/3/', 'http://127.0.0.1:8790/people/4/', 'http://127.0.0.1:8790/people/5/', 'http://127.0.0.1:8790/people/6/', 'http://127.0.0.1:8790/people/7/', 'http://127.0.0.1:8790/people/10/', 'http://127.0.0.1:8790/people/11/', 'http://127.0.0.1:8790/people/12/', 'http://127.0.0.1:8790/people/13/', 'http://127.0.0.1:8790/people/20/', 'http://127.0.0.1:8790/people/21/', 'http://127.0.0.1:8790/people/33/', 'http://127.0.0.1:8790/people/35/', 'http://127.0.0.1:8790/people/46/', 'http://127.0.0.1:8790/people/51/', 'http://127.0.0.1:8790/people/52/', 'http://127.0.0.1:8790/people/53/', 'http://127.0.0.1:8790/people/54/', 'http://127.0.0.1:8790/people/55/', 'http://127.0.0.1:8790/people/56/', 'http://127.0.0.1:8790/people/58/', 'http://127.0.0.1:8790/people/63/', 'http://127.0.0.1:8790/people/64/', 'http://127.0.0.1:8790/people/67/', 'http://127.0.0.1:8790/people/68/', 'http://127.0.0.1:8790/people/75/', 'http://127.0.0.1:8790/people/78/', 'http://127.0.0.1:8790/people/79/', 'http://127.0.0.1:8790/people/80/', 'http://127.0.0.1:8790/people/81/', 'http://127.0.0.1:8790/people/82/', 'http://127.0.0.1:8790/people/83/'],
    'planets': ['http://127.0.0.1:8790/planets/1/', 'http://127.0.0.1:8790/planets/2/', 'http://127.0.0.1:8790/planets/5/', 'http://127.0.0.1:8790/planets/8/', 'http://127.0.0.1:8790/planets/9/', 'http://127.0.0.1:8790/planets/12/', 'http://127.0.0.1:8790/planets/13/', 'http://127.0.0.1:8790/planets/14/', 'http://127.0.0.1:8790/planets/15/', 'http://127.0.0.1:8790/planets/16/', 'http://127.0.0.1:8790/planets/17/', 'http://127.0.0.1:8790/planets/18/', 'http://127.0.0.1:8790/planets/19/'],
    'starships': ['http://127.0.0.1:8790/starships/2/', 'http://127.0.0.1:8790/starships/32/', 'http://127.0.0.1:8790/starships/48/', 'http://127.0.0.1:8790/starships/59/', 'http://127.0.0.1:8790/starships/61/', 'http://127.0.0.1:8790/starships/63/', 'http://127.0.0.1:8790/starships/64/', 'http://127.0.0.1:8790/starships/65/', 'http://127.0.0.1:8790/starships/66/', 'http://127.0.0.1:8790/starships/68/', 'http://127.0.0.1:8790/starships/74/', 'http://127.0.0.1:8790/starships/75/'],
    'vehicles': ['http://127.0.0.1:8790/vehicles/33/', 'http://127.0.0.1:8790/vehicles/50/', 'http://127.0.0.1:8790/vehicles/53/', 'http://127.0.0.1:8790/vehicles/56/', 'http://127.0.0.1:8790/vehicles/60/', 'http://127.0.0.1:8790/vehicles/62/', 'http://127.0.0.1:8790/vehicles/67/', 'http://127.0.0.1:8790/vehicles/69/', 'http://127.0.0.1:8790/vehicles/70/', 'http://127.0.0.1:8790/vehicles/71/', 'http://127.0.0.1:8790/vehicles/72/', 'http://127.0.0.1:8790/vehicles/73/', 'http://127.0.0.1:8790/vehicles/76/'], 
    'species': ['http://127.0.0.1:8790/species/1/', 'http://127.0.0.1:8790/species/2/', 'http://127.0.0.1:8790/species/3/', 'http://127.0.0.1:8790/species/6/', 'http://127.0.0.1:8790/species/15/', 'http://127.0.0.1:8790/species/19/', 'http://127.0.0.1:8790/species/20/', 'http://127.0.0.1:8790/species/23/', 'http://127.0.0.1:8790/species/24/', 'http://127.0.0.1:8790/species/25/', 'http://127.0.0.1:8790/species/26/', 'http://127.0.0.1:8790/species/27/', 'http://127.0.0.1:8790/species/28/', 'http://127.0.0.1:8790/species/29/', 'http://127.0.0.1:8790/species/30/', 'http://127.0.0.1:8790/species/33/', 'http://127.0.0.1:8790/species/34/', 'http://127.0.0.1:8790/species/35/', 'http://127.0.0.1:8790/species/36/', 'http://127.0.0.1:8790/species/37/'],
    'created': '2014-12-20T18:49:38.403000Z', 'edited': '2014-12-20T20:47:52.073000Z', 'url': 'http://127.0.0.1:8790/films/6/'
}
"""

from datetime import datetime, timedelta
import requests
import json
import threading

# Include cse 251 common Python files
from cse251 import *

# Const Values
TOP_API_URL = "http://127.0.0.1:8790"
FILM = 6

# Global Variables
call_count = 0
lock = threading.Lock()


# TODO Add your threaded class definition here
class RequestThread(threading.Thread):
    # TODO - Add code to make an API call and return the results
    def __init__(self, url):
        threading.Thread.__init__(self)
        self.url = url

    def run(self):
        global lock
        global call_count 
        with lock:
            call_count += 1

        response = requests.get(self.url)

        if response.status_code != 200:
            self.success=False
            print(f"requests.get({self.url})\nresponse.status_code != 200")
            return

        self.success = True
        self.data = response.json()

        if "name" in self.data:
            self.name = self.data["name"]


# TODO Add any functions you need here
def list_to_threads(urls: list):
    threads = []
    for url in urls:
        threads.append(RequestThread(rf"{url}"))
    return threads

def start_threads(threads: list):
    for thread in threads:
        thread.start()

def join_threads(threads: list):
    for thread in threads:
        thread.join()

def main():
    log = Log(show_terminal=True)
    log.start_timer("Starting to retrieve data from the server")
    log.write("-----------------------------------------")

    # TODO Retrieve Top API urls
    top = RequestThread(TOP_API_URL)
    top.start()
    top.join()
    if not top.success: return
    # log.write(top.data)

    six = RequestThread(rf"{top.data['films']}{FILM}")
    six.start()
    six.join()
    if not six.success: return
    # log.write(six.data)

    title = six.data["title"]
    director = six.data["director"]
    producer = six.data["producer"]
    released = six.data["release_date"]

    urls_dict = {}
    for key in six.data:
        if isinstance(six.data[key], list) and "http" in six.data[key][0]:
            urls_dict[key] = six.data[key]

    # print(urls_dict)

    # characters = six.data["characters"]
    # planets = six.data["planets"]
    # starships = six.data["starships"]
    # vehicles = six.data["vehicles"]
    # species = six.data["species"]
    
    # data = {"characters": characters, "planets": planets, "starships": starships, "vehicles": vehicles, "species": species}

    # TODO Retireve Details on film 6
    threads_dict = {}
    for key in urls_dict.keys():
        threads_dict[key] = list_to_threads(urls_dict[key])

    # print(threads_dict)

    for threads in threads_dict.values():
        start_threads(threads)
        
    for threads in threads_dict.values():
        join_threads(threads)

    results_dict = {}
    for key in threads_dict.keys():
        results_dict[key] = []
        for thread in threads_dict[key]:
            if thread.success: results_dict[key].append(thread.name)
        results_dict[key].sort()

    strings_dict = {}
    for key in results_dict.keys():
        strings_dict[key] = ""
        for result in results_dict[key]:
            strings_dict[key] += result + ", "
        strings_dict[key] = strings_dict[key][0:-2]

    # for key in results_dict.keys():
        # print(results_dict[key][0])

    # TODO Display results
    log.write(f"Title: {title}")
    log.write(f"Director: {director}")
    log.write(f"Producer: {producer}")
    log.write(f"Released: {released}\n")

    log.write(f"Characters: {len(results_dict['characters'])}")
    log.write(f"{strings_dict['characters']}\n")

    log.write(f"Planets: {len(results_dict['planets'])}")
    log.write(f"{strings_dict['planets']}\n")

    log.write(f"Starships: {len(results_dict['starships'])}")
    log.write(f"{strings_dict['starships']}\n")

    log.write(f"Vehicles: {len(results_dict['vehicles'])}")
    log.write(f"{strings_dict['vehicles']}\n")

    log.write(f"Species: {len(results_dict['species'])}")
    log.write(f"{strings_dict['species']}\n")

    log.stop_timer("\nTotal Time To complete")
    log.write(f"There were {call_count} calls to the server")
    

if __name__ == "__main__":
    main()
