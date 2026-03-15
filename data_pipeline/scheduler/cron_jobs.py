import schedule
import time
from scheduler.pipeline_runner import run_pipeline

def start():

    schedule.every(1).hours.do(run_pipeline)

    while True:

        schedule.run_pending()

        time.sleep(60)
