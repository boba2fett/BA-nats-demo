# BA NATS Demo

Dies ist eine kleine Demo Anwendung, die demonstrieren soll wie NATS in einem Service-Worker Konstrukt zur Verarbeitung von Jobs eingesetzt werden kann. Der Service nimmt Jobs per HTTP an, versendet eine Nachricht in einen Stream und der Worker bearbeitet die Jobs. Zusätzlich gibt es noch einen Dead Letter Queue Worker, der vom Worker nicht erfolgreich verarbeitete Jobs übernimmt.

## Beispiel

Zum Nachvollziehen der Beispiele werden Git (`git`), Docker (`docker`), cURL (`curl`) und die NATS CLI (`nats`) benötigt.

1. Git-Repository clonen `git clone https://github.com/boba2fett/BA-nats-demo`
2. NATS Server starten `docker compose up -d` im Verzeichnis `nats-server` ausführen

Der NATS Server kann nach den Beispielen per `docker compose down` wieder heruntergefahren werden.

### Verarbeitung durch zwei Worker

Beide Worker nutzen den selben Consumer und sollen sich den Workload aufteilen.

1. Umgebung mit zwei Workern starten `docker compose up --scale worker=2`
2. In einem anderen Kontext ein paar Jobs anlegen `for i in ‘seq 1 20‘; do curl -XPOST -H "Content-type: application/json" -d ’{"payload": "nothing"}’ ’http://localhost:8000/job’; done`

Es ist nun zu beobachten, dass sich die Worker abwechseln. Im Object Store sammeln sich die Jobs an. Diese können per `nats object ls job` eingesehen werden. Eine Übersicht der Streams wird per `nats stream ls` angezeigt.

### Erneute Zustellung bis zur Dead Letter Queue

Um zu simulieren, dass der Worker den Job nicht verabeiten kann, wird im Payload beim Anlegen des Jobs `retry` mitgegeben.

1. Umgebung starten `docker compose up`
2. In einem anderen Kontext einen Job anlegen `curl -XPOST -H "Content-type: application/json" -d ’{"payload": "retry"}’ ’http://localhost:8000/job’`

Nun ist zu beobachten, dass der Worker die Nachricht fünf Mal erhält, bis diese an den Dead Letter Queue Worker versendet wird. Eine Übersicht der Streams per `nats stream ls` zeigt, dass die ursprüngliche Nachricht im Stream `newJob` enthalten bleibt.
