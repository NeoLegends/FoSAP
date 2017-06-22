# H13 - First Longest Match

Dies ist ein Programm, welches die First-Longest-Match-Strategie für Regexes nach Aufgabe H13 implementiert und demonstriert.

In H13 wurde eine vereinfachte Version der Hausaufgabe präsentiert, die darauf aufbaut, dass neben den beiden gegebenen regulären Ausdrücken rID und rNUM, nur ganze Wörter als Reguläre Ausdrücke vorkommen sollen (also keine Kleen'schen Hüllen, Oder-Verknüpfungen, etc.). Denn dann ist es möglich, einen bereits bestehenden DFA sehr einfach mit Match-Funktionalität für die gegebenen Ausdrücke zu erweitern, ohne dabei über die Potenzmengenkonstruktion und Minifizierung, etc. gehen zu müssen. Diese Version der Aufgabe wird hier präsentiert.

Da im Aufgabentext geschrieben steht, dass rID und rNUM immer als Input-Regex vorkommen, wurden diese als "handoptimierter" DFA ins Programm hartgecoded. Somit sind sie immer als Input gegeben, wir ersparen uns aber das Parsen und verarbeiten.

Zusätzlich wurde, um die Implementation möglichst sauber und einfach zu gestalten, eine weitere Vereinfachung eingeführt: Es wird nicht mit einem strikten DFA gearbeitet. Stattdessen verwenden wir einen NFA, der anstelle von "Senkenzuständen" einfach fehlende Transitionen hat. Soll also von einem Zustand aus über ein gegebenes Symbol nie wieder ein Endzustand erreicht werden, wird einfach keine Transition von diesem Zustand über dieses Symbol in den Automaten eingefügt. Ansonsten verhält sich dieser NFA genau wie ein DFA. Er kann sich zu jedem möglichen Zeitpunkt in nur _einem_ Zustand befinden, bzw. von einem Zustand aus gibt es jeweils nur _eine_ Transition über ein gegebenes Symbol in einen anderen. Damit bleibt die Geschwindigkeit bei der eines DFAs und wir kriegen keine Zustandsexplosion wie bei normalen NFAs möglich wären.

Diese Vereinfachungen erlauben es, den Automaten mit nur sehr wenig Aufwand um gegebene Wörter zu erweitern. Und zwar "simulieren" wir das einzufügene Wort vom Startzustand aus solange bis eine Transition fehlt, und wir in einen Senkenzustand übergehen würden. Hier fügen wir einfach die fehlende Transition + Zustand ein, und simulieren weiter. So wird dann das Wort einfach als "Kette" (oder Zweig) an die passende Stelle in den Automaten eingefügt. Sind alle Zustände eingefügt, wird zuletzt noch der letzte Zustand als Endzustand markiert, denn er soll ja das gegebene Wort treffen. Da wir durch obige Vereinfachung nicht auf Schleifen oder Senken achten müssen, wird der Algorithmus dafür extrem einfach. Nachteilig wirkt sich aus, dass Ausdrücke, die in anderen, bereits hinzugefügten regulären Ausdrücken enthalten sind, nicht mehr von diesen zu unterscheiden sind, weil einfach deren Transitionen "mitbenutzt" werden. Dies ist in den Testergebnissen zu erkennen. `for` wird durch die Ausdrücke `FOR` und `rID` gleichzeitig erkannt, weil der Ausdruck für beide eine gültige Eingabe ist.

## Projekt

Die Quelldateien befinden sich im `src/`-Ordner. `lib.rs` ist die Hauptdatei. Sie enthält die Simulationsfunktion und die Tests.
In `dfa.rs` befindet sich die Implementation der "Erweiterungs"-Funktionalität, sowie der eigentliche First Longest Match-Algorithmus.
Im Code selber finden sich erklärende Kommentare, die naher an der Implementation verfasst sind.

Unsere Testergebnisse finden sich in `TESTRESULTS.txt` neben dieser Readme. Sie sind direkt aus der Ausgabe von `cargo test -- --nocapture` entstanden. In der Ausgabe wird jeweils die aktuelle Eingabe, die verwendeten Regular Expressions (mit dem jeweiligen Token) und die erkannten Prefixes angezeigt. Zu den dort angegebenen Regulären Ausdrücken ist jeweils noch `rNUM` und `rID` dazuzuzählen. Sie sind bei jedem Test mit dabei, werden aber nicht in den Testergebnissen angezeigt, weil sie ins Programm hardcoded sind.

## Ausführen

Die gegebenen Tests und Beispiele können in diesem Ordner mittels

```bash
cargo test
```

kompiliert und ausgeführt werden. Will man die Testausgabe betrachten, ist zusätzlich

```bash
cargo test -- --nocapture
```

vonnöten, da der Test-Harness standardmäßig die Stdout-Ausgabe verschluckt. Idealerweise würde man jetzt "richtige" Tests (und nicht nur Testausführungen) hinzufügen, deren Ergebnisse nach der Ausführung noch maschinell validiert werden. Dann wäre eine händische Betrachtung der Ausgabe nicht mehr nötig.