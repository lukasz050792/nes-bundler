#include "signalsmith-stretch-wrapper.hpp"
std::unique_ptr<SignalsmithStretch> signalsmith_stretch_new(int nChannels, float sampleRate) {
  SignalsmithStretch *instance = new SignalsmithStretch();
  instance->presetCheaper(nChannels, sampleRate);
  return std::unique_ptr<SignalsmithStretch>(instance);
}

void signalsmith_stretch_process(std::unique_ptr<SignalsmithStretch> &ptr, __SampleFormat **input, int nInputSamples, __SampleFormat **output, int nOutputSamples) {
  ptr->process(input, nInputSamples, output, nOutputSamples);
}