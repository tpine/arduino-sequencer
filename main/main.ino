int buttons[] = {0,};

int notes[] = {261, 293, 329, 349, 391, 440, 493};

int sequence[] = {1, 2, 3, 4};

int sequencePos = 0;

int prevPotSel = 0;

unsigned long prevTime = 0;

int interval = 300;

int sensorLow = 1023;
int sensorHigh = 0;


void setup() {
	/* for (int i; i <= 3; i++) { */
	/* 	pinMode(i+2, OUTPUT); */
	/* 	digitalWrite(i+2, LOW);	 */
	/* } */
}

void loop() {
	unsigned long currentTime = millis();
	int keyVal = analogRead(A0);
	interval = analogRead(A2);

	if (keyVal >= 1023) {
		prevPotSel = sequence[0];
		sequence[0] = map(analogRead(A1), 0, 1023, 0, 7);
		
		if (sequence[0] != prevPotSel) {
			tone(11, sequence[0]);
		}
	} else if (keyVal >= 990 && keyVal <= 1010) {
		prevPotSel = sequence[1];
		sequence[1] = map(analogRead(A1), 0, 1023, 0, 7);
		if (sequence[1] != prevPotSel) {
			tone(11, sequence[1]);
		}
	} else if (keyVal >= 550 && keyVal <= 515) {
		prevPotSel = sequence[2];
		sequence[2] = map(analogRead(A1), 0, 1023, 0, 7);
		if (sequence[2] != prevPotSel) {
			tone(11, sequence[2]);
		}
	} else if (keyVal >= 5 && keyVal <= 10) {
		prevPotSel = sequence[3];
		sequence[3] = map(analogRead(A1), 0, 1023, 0, 7);
		if (sequence[3] != prevPotSel) {
			tone(11, sequence[3]);
		}
	} else {
		if ((unsigned long) (currentTime - prevTime) >= interval) {
			tone(11, notes[sequence[sequencePos]]);
	
			// Iterate position in sequence avoiding overflow
			if (sequencePos != 3) {
				sequencePos = sequencePos + 1;
			} else {
				sequencePos = 0;
			}

			// Wait one sec
			prevTime = currentTime;
		}			
	}
	/* for (int i; i <= 3; i++) { */
	/* 	if(sequencePos == i) { */
	/* 		digitalWrite(i+2, HIGH); */
	/* 	} else { */
	/* 		digitalWrite(i+2, LOW); */
	/* 	} */
		
	/* } */
	

}
