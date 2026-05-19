import { kernelClient } from "../../os/kernel/kernelClient";

export type AudioOutput = {
  id: string;
  label: string;
  active: boolean;
};

export type AudioStatus = {
  volume: number;
  muted: boolean;
  outputs: AudioOutput[];
  activeOutputId: string;
};

export const audioKernel = {
  async status() {
    const response = await kernelClient.request<AudioStatus>({
      type: "audio.status",
      data: {}
    });
    if (!response.data) throw new Error("audio_status_missing");
    return response.data;
  },
  async setVolume(volume: number) {
    const response = await kernelClient.request<AudioStatus>({
      type: "audio.volume",
      data: { volume }
    });
    if (!response.data) throw new Error("audio_volume_missing");
    return response.data;
  },
  async setOutput(outputId: string) {
    const response = await kernelClient.request<AudioStatus>({
      type: "audio.setOutput",
      data: { outputId }
    });
    if (!response.data) throw new Error("audio_output_missing");
    return response.data;
  }
};
