![Kestrel-Queue](assets/kestrel-enqueue-rs.jpeg)

# Kestrel Queue RS

Hai mai avuto bisogno di gestire **lavori in background** in modo affidabile senza complicarti troppo la vita? **Kestrel Queue RS** è un piccolo sistema di task queue scritto in Rust, pensato per chi vuole un flusso di lavoro semplice ma solido, con monitoraggio e gestione automatica degli errori.

### Perché usarlo

- Eviti di reinventare la ruota per job scheduling e retry/backoff.
- Hai una **persistenza sicura** dello stato dei job in Postgres.
- Puoi **monitorare le metriche** in tempo reale grazie a Prometheus.
- CI pronta su GitHub Actions per test e build automatici, senza dipendere da Docker.

### Caratteristiche principali

- **API REST per enqueue dei job**
    
    Inserisci lavori da eseguire e persistili direttamente in **Postgres**.
    
- **Worker robusti**
    
    Consuma i job, gestisce automaticamente retry e backoff in caso di errori, e aggiorna lo stato direttamente sul database.
    
- **Monitoraggio e metriche**
    
    Dashboard minimale con metriche in stile **Prometheus** per tenere sotto controllo attività e performance.
    
- **Continuous Integration**
    
    CI pronta su **GitHub Actions**, senza bisogno di Docker.