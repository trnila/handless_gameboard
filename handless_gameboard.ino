const byte KEY_JOYSTICK_BIT = 6;
const byte KEY_PRESSED_BIT = 7;

struct key {
  int pin;
  int state;
};

struct joystick {
  int pin;
  int center;
  int threshold;
  int state;
};

struct key keys[] = {
  {12},
  {11},
  {10},
  //{9},
};

struct joystick joysticks[] = {
  {
    .pin = A0,
    .center = 500,
    .threshold = 300    
  },
  {
    .pin = A1,
    .center = 500,
    .threshold = 300    
  },  
};

void setup() {
  Serial.begin(9600);

  for(int i = 0; i < sizeof(keys)/sizeof(*keys); i++) {
    pinMode(keys[i].pin, INPUT_PULLUP);
    keys[i].state = HIGH;
  }
}

void loop() {
  for(int i = 0; i < sizeof(keys)/sizeof(*keys); i++) {
    int cur = digitalRead(keys[i].pin);
    if(cur != keys[i].state) {
      Serial.write((char) (i | (!cur << KEY_PRESSED_BIT)));
      keys[i].state = cur;
    }
  }

  for(int i = 0; i < sizeof(joysticks)/sizeof(*joysticks); i++) {
    int cur = analogRead(joysticks[i].pin);
    int key = -1;
    if(abs(cur - joysticks[i].center) > joysticks[i].threshold) {
      key = cur - joysticks[i].center > 0;
    }

    if(key != joysticks[i].state) {
      // release key
      if(joysticks[i].state >= 0) {
        Serial.write((2 * i + joysticks[i].state) | (1 << KEY_JOYSTICK_BIT));
      }

      // press key
      if (key >= 0) {
        Serial.write((2 * i + key) | (1 << KEY_JOYSTICK_BIT) | (1 << KEY_PRESSED_BIT));
      }
      
      joysticks[i].state = key;
    }
  }  
}
