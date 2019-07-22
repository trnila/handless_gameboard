struct key {
  int pin;
  int state;
};

struct key keys[] = {
  {12},
  {11},
  {10},
  {9},
};

void setup() {
  Serial.begin(9600);

  for(int i = 0; i < sizeof(keys)/sizeof(*keys); i++) {
    pinMode(keys[i].pin, INPUT);
    keys[i].state = HIGH;
  }
}

void loop() {
  for(int i = 0; i < sizeof(keys)/sizeof(*keys); i++) {
    int cur = digitalRead(keys[i].pin);
    if(cur != keys[i].state) {
      Serial.write((char) (i | (!cur << 7)));
      keys[i].state = cur;
    }
  }
}
