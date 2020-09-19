#pragma once
#include <iostream>
#include <vector>
#include <fstream>
#include <assert.h>
#include "Neuron.h"
using namespace std;
class FNet {
public:
	FNet(const vector<unsigned> &toplogy);
	FNet(ifstream &fin);
	void storeNet(ofstream &fout);
	FNet() {}
	void feedForward(const vector<double> &inputVals);
	void feedForward(const vector<vector<double>> &inputVals);
	void backProp(const vector<double> &targetVals);
	void getResults(vector<double> &resultVals) const;
	void getLayer(vector<Neuron> &layer);

private:
	vector<Layer> m_layers; //m_layer[layerNumber][neuronInLayer]
};
FNet::FNet(ifstream &fin) {
	int layerNum = 0;
	fin >> layerNum;
	vector<unsigned> topology;
	for (int l = 0; l < layerNum; l++) {
		int num;
		fin >> num;
		topology.push_back(num);
	}
	for (int layer = 0; layer < layerNum; layer++) {
		m_layers.push_back(Layer());
		unsigned numOutputs = layer == topology.size() - 1 ? 0 : topology[layer + 1] - 1;
		for (int n = 0; n < topology[layer]; n++) {
			int index;
			fin >> index;
			vector<double> weights;
			for (int w = 0; w < numOutputs; w++) {
				double outW;
				fin >> outW;
				weights.push_back(outW);
			}
			m_layers.back().push_back(Neuron(weights, index));
			//cout << "add neuron for layer " << layer << endl;
			if (n == topology[layer] - 1) {
				double output = 0;
				fin >> output;
				m_layers.back().back().setOutputVal(output);
			}
		}
	}
}
void FNet::getLayer(vector<Neuron> &layer) {
	layer.clear();
	for (int n = 0; n < m_layers[0].size(); n++) {
		layer.push_back(m_layers[0][n]);
	}
}
void FNet::getResults(vector<double> &resultVals) const {
	resultVals.clear();
	for (unsigned n = 0; n < m_layers.back().size() - 1; n++) {
		resultVals.push_back(m_layers.back()[n].getOutputVal());
	}
}
void FNet::backProp(const vector<double> &targetVals) {
	//calculated overall net error
	Layer &outputLayer = m_layers.back();
	//calc output gradients
	for (unsigned n = 0; n < outputLayer.size() - 1; n++) {
		Layer &prevLayer = m_layers[m_layers.size() - 2];
		outputLayer[n].calcOutputGradients(targetVals[n]);
		//outputLayer[n].updateInputWeights(prevLayer);
	}
	//calc hidden gradients
	for (unsigned layerNum = m_layers.size() - 2; layerNum > 0; layerNum--) {
		Layer &hiddenLayer = m_layers[layerNum];
		Layer &nextLayer = m_layers[layerNum + 1];
		Layer &prevLayer = m_layers[layerNum - 1];
		for (unsigned n = 0; n < hiddenLayer.size() - 1; n++) {
			hiddenLayer[n].calcHiddenGradients(nextLayer);
			//hiddenLayer[n].updateInputWeights(prevLayer);
		}
	}
	for (unsigned layerNum = m_layers.size() - 1; layerNum > 0; layerNum--) {
		Layer &prevLayer = m_layers[layerNum - 1];
		for (unsigned n = 0; n < m_layers[layerNum].size() - 1; n++) {
			m_layers[layerNum][n].updateInputWeights(prevLayer);
		}
	}
	for (unsigned n = 0; n < m_layers[0].size() - 1; n++) {
		m_layers[0][n].calcHiddenGradients(m_layers[1]);
	}
}
void FNet::feedForward(const vector<vector<double>> &inputVals) {
	assert(inputVals.size()*inputVals[0].size() == m_layers[0].size() - 1); //makes sure number of inputs and equal to number of input neurons
	//makes input neurons output a certain data input
	for (unsigned i = 0; i < inputVals.size(); i++) {
		for (int j = 0; j < inputVals[i].size(); j++) {
			m_layers[0][(i*inputVals[i].size()) + j].setOutputVal(inputVals[i][j]);
		}
	}
	for (unsigned layerNum = 1; layerNum < m_layers.size(); layerNum++) {
		Layer &prevLayer = m_layers[layerNum - 1];
		for (unsigned n = 0; n < m_layers[layerNum].size() - 1; n++) {
			m_layers[layerNum][n].feedForward(prevLayer);
		}
	}
}
void FNet::feedForward(const vector<double> &inputVals) {
	assert(inputVals.size() == m_layers[0].size() - 1); //makes sure number of inputs and equal to number of input neurons
	//makes input neurons output a certain data input
	for (unsigned i = 0; i < inputVals.size(); i++) {
		m_layers[0][i].setOutputVal(inputVals[i]);
	}
	for (unsigned layerNum = 1; layerNum < m_layers.size(); layerNum++) {
		Layer &prevLayer = m_layers[layerNum - 1];
		for (unsigned n = 0; n < m_layers[layerNum].size() - 1; n++) {
			m_layers[layerNum][n].feedForward(prevLayer);
		}
	}
}
FNet::FNet(const vector<unsigned> &topology) {
	unsigned numLayers = topology.size();
	for (unsigned layerNum = 0; layerNum < numLayers; layerNum++) {
		m_layers.push_back(Layer()); //adds a layer
		unsigned numOutputs = layerNum == topology.size() - 1 ? 0 : topology[layerNum + 1];//if layerNum is the output layer then set to 0 otherwise set to number of neurons in next layer
		for (unsigned neuronNum = 0; neuronNum < topology[layerNum]; neuronNum++) {
			m_layers.back().push_back(Neuron(numOutputs, neuronNum));
			cout << "added neuron for layer [" << layerNum << "] neuronNum: " << neuronNum  << endl;
		}
		m_layers.back().back().setOutputVal(rand() / double(RAND_MAX));
	}
}
void FNet::storeNet(ofstream &fout) {
	fout << m_layers.size() << endl;
	for (int l = 0; l < m_layers.size(); l++) {
		fout << m_layers[l].size() << endl;
	}
	for (int l = 0; l < m_layers.size(); l++) {
		for (int n = 0; n < m_layers[l].size(); n++) {
			fout << n << " ";
			vector<double> weights;
			m_layers[l][n].storeNeuron(weights);
			for (int w = 0; w < weights.size(); w++) {
				fout << weights[w] << " ";
			}
			if (n == m_layers[l].size() - 1) {
				fout << m_layers[l][n].getOutputVal();
			}
			fout << endl;
		}
	}
}