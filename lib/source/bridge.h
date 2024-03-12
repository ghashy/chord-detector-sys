#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif
    typedef struct {
            void* ptr;
            size_t len;
    } c_vec;

    typedef struct {
            int root_note;
            int quality;
            int intervals;
    } chord_info;

    // ───── C_Chromagram ─────────────────────────────────────────────────────── //

    typedef void* C_Chromagram;

    /// Constructor
    C_Chromagram* init_chromagram(int frameSize, int sampleRate);
    /// Destructor
    void free_chromagram(C_Chromagram*);

    void set_input_audio_framesize(C_Chromagram*, int);
    void set_sampling_frequency(C_Chromagram*, int);
    void set_chroma_calculation_interval(C_Chromagram*, int);
    void process_audio_frame(C_Chromagram*, double*);
    int is_ready(C_Chromagram*);
    /// Returns borrowed chroma vector
    c_vec get_chromagram(C_Chromagram*);

    // ───── C_ChordDetector ──────────────────────────────────────────────────── //

    typedef void* C_ChordDetector;

    /// Constructor
    C_ChordDetector* init_chord_detector();
    /// Destructor
    void free_chord_detector(C_ChordDetector*);

    /// Takes reference to data, and makes deep copy of it
    void detect_chord(C_ChordDetector*, c_vec);
    chord_info fetch(C_ChordDetector*);

#ifdef __cplusplus
}
#endif
