#include <stdio.h>
#include <stdlib.h>

#include <bridge.h>

int main() {
  int frameSize = 512;
	int sampleRate = 44100;
  double frame[frameSize];
  C_Chromagram c = init_chromagram(frameSize, sampleRate);
  process_audio_frame(c, frame);

  if (is_ready(c)) {
    printf("ready");
  } else {
    printf("not ready");
  }

  C_ChordDetector d = init_chord_detector();

  free_chord_detector(d);
  free_chromagram(c);


  return EXIT_SUCCESS;
}
