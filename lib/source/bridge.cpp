#define USE_KISS_FFT

#include "ChordDetector.h"
#include <Chromagram.h>
#include <bridge.h>
#include <iostream>

extern "C" {
    C_Chromagram* init_chromagram(int frameSize, int sampleRate) {
        C_Chromagram* ptr = (C_Chromagram*)new Chromagram(frameSize, sampleRate);
        return ptr;
    }

    void free_chromagram(C_Chromagram* o) {
        delete (Chromagram*)o;
    }

    void set_input_audio_framesize(C_Chromagram* o, int framesize) {
        ((Chromagram*)o)->setInputAudioFrameSize(framesize);
    }

    void set_sampling_frequency(C_Chromagram* o, int frequency) {
        ((Chromagram*)o)->setSamplingFrequency(frequency);
    }

    void set_chroma_calculation_interval(C_Chromagram* o, int interval) {
        ((Chromagram*)o)->setChromaCalculationInterval(interval);
    }

    void process_audio_frame(C_Chromagram* o, double* frame) {
        ((Chromagram*)o)->processAudioFrame(frame);
    }

    int is_ready(C_Chromagram* o) {
        return ((Chromagram*)o)->isReady();
    }

    c_vec get_chromagram(C_Chromagram* o) {
        std::vector<double> chroma = ((Chromagram*)o)->getChromagram();
        size_t size = chroma.size();
        double* ptr = new double[chroma.size()];
        std::copy(chroma.begin(), chroma.end(), ptr);
        c_vec vec = {ptr, size};
        return vec;
    }

    C_ChordDetector* init_chord_detector() {
        return (C_ChordDetector*)new ChordDetector();
    }

    void free_chord_detector(C_ChordDetector* o) {
        delete (ChordDetector*)o;
    }

    void detect_chord(C_ChordDetector* o, c_vec chroma) {
        const double* buffer = (double*)chroma.ptr;
        std::vector<double> v(buffer, buffer + chroma.len);
        ((ChordDetector*)o)->detectChord(v);
    }

    chord_info fetch(C_ChordDetector* o) {
        int intervals = ((ChordDetector*)o)->intervals;
        int quality = ((ChordDetector*)o)->quality;
        int root_note = ((ChordDetector*)o)->rootNote;
        chord_info chord_info = {root_note, quality, intervals};
        return chord_info;
    }
}

