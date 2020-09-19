#pragma once
#include <iostream>
#include <vector>
#include <fstream>
#include <assert.h>
using namespace std;
struct Connection {
	double weight;
	double deltaWeight;
};
class Neuron;
typedef vector<Neuron> Layer;
class Neuron {
public:
	Neuron(unsigned numOutputs, unsigned myIndex);
	Neuron(vector<double> &outputWeights, unsigned myIndex);
	Neuron();
	Neuron(unsigned pX, unsigned pY, int c);
	void setOutputVal(double val) { n_outputVal = val; }
	double getOutputVal(void) const { return n_outputVal; }
	void feedForward(const Layer &prevLayer);
	void feedForward(vector<vector<Neuron>> &prevLayer, vector<vector<double>> &fW, double &bias);
	void calcOutputGradients(double targetVal);
	void calcHiddenGradients(const Layer &nextLayer);
	double sumDOW(const Layer &nextLayer) const;
	void updateInputWeights(Layer &prevLayer);
	void updateInputWeights(vector<vector<Neuron>> &prevLayer, vector<vector<double>> &fW, double &bias, int currentLayerSize);
	void updatePrevLayerGradient(vector<vector<Neuron>> &prevLayer, vector<vector<double>> &fW);
	void setGradient(double grad) { n_gradient = grad; }
	double getGradient() { return n_gradient; }
	unsigned getConvX() { return convX; }
	unsigned getConvY() { return convY; }
	void setConvCoord(unsigned x, unsigned y) { convX = x; convY = y; }
	void storeNeuron(vector<double> &weights);
	void addGradient(double grad) { n_gradient += grad; }
private:
	static double transferFunction(double x);
	static double transferFunctionDerivative(double x);
	static double randomWeight(void) { return rand() / (double(RAND_MAX)); }
	static double eta; // 0.0 to 1.0
	static double alpha; //0.0 to n
	static int batchSize;
	double n_outputVal;
	int batchCount;
	vector<Connection> n_outputWeights;
	unsigned n_myIndex;
	double n_gradient;
	vector<double> storeGradient;
	//conv neuron
	unsigned posX, posY; //start pos on prev layer
	double biasGrad;
	//pool neuron
	unsigned convX, convY;
};
double Neuron::eta = 0.015;
double Neuron::alpha = 0.5;
int Neuron::batchSize = 1;
void Neuron::updateInputWeights(Layer &prevLayer) {
	for (unsigned n = 0; n < prevLayer.size(); n++) {
		Neuron &neuron = prevLayer[n];
		if (storeGradient.size() < n + 1) {
			storeGradient.push_back(0.0);
		}
		storeGradient[n] += neuron.getOutputVal()*n_gradient;
		batchCount++;
		if (batchCount == ((batchSize - 1)*prevLayer.size()) + n + 1) {
			neuron.n_outputWeights[n_myIndex].weight += eta * storeGradient[n] / batchSize;
			storeGradient[n] = 0.0;
		}
	}
	if (batchCount == batchSize * prevLayer.size()) {
		batchCount = 0;
	}
}
void Neuron::updateInputWeights(vector<vector<Neuron>> &prevLayer, vector<vector<double>> &fW, double &bias, int currentLayerSize) {
	for (int y = 0; y < fW.size(); y++) {
		for (int x = 0; x < fW[y].size(); x++) {
			if (storeGradient.size() < (fW[0].size()*y) + x + 1) {
				storeGradient.push_back(0.0);
			}
			storeGradient[(y*fW[0].size()) + x] += prevLayer[posY + y][posX + x].getOutputVal() * n_gradient;
			batchCount++;
			if (batchCount == (batchSize - 1)*(fW.size()*fW[0].size()) + (y*fW[0].size()) + x + 1) {
				fW[y][x] += eta * storeGradient[(y*fW[0].size()) + x] / (batchSize);
				storeGradient[(y*fW[0].size()) + x] = 0.0;
			}
		}
	}
	biasGrad += n_gradient;
	if (batchCount == batchSize * fW.size() * fW[0].size()) {
		bias += eta * biasGrad / (batchSize * currentLayerSize);
		biasGrad = 0;
		batchCount = 0;
	}
}
double Neuron::sumDOW(const Layer &nextLayer) const {
	double sum = 0.0;
	for (unsigned n = 0; n < nextLayer.size() - 1; n++) {
		sum += n_outputWeights[n].weight * nextLayer[n].n_gradient;
	}
	return sum;
}
void Neuron::calcHiddenGradients(const Layer &nextLayer) {
	double dow = sumDOW(nextLayer);
	n_gradient = dow * Neuron::transferFunctionDerivative(n_outputVal);
}
void Neuron::calcOutputGradients(double targetVals) {
	//n_gradient = ((targetVals / n_outputVal) - ((1 - targetVals) / (1 - n_outputVal))) * Neuron::transferFunctionDerivative(n_outputVal); //sigmoid cross entropy cost func
	//n_gradient = (targetVals - n_outputVal) * Neuron::transferFunctionDerivative(n_outputVal); //quad cost func
	n_gradient = targetVals - n_outputVal; //cross entropy cost func for both sigmoid and tanh
}
double Neuron::transferFunction(double x) {
	//return tanh(x);
	return 1.0 / (1.0 + exp(-x));
}
double Neuron::transferFunctionDerivative(double x) {
	//return 1.0-(x*x); //aprox. for domain 
	return  x - (x*x);
}
void Neuron::feedForward(const Layer &prevLayer) {
	double sum = 0.0;
	for (unsigned n = 0; n < prevLayer.size(); n++) {
		sum += prevLayer[n].getOutputVal() * prevLayer[n].n_outputWeights[n_myIndex].weight;
	}
	n_outputVal = Neuron::transferFunction(sum); //activation function
}
void Neuron::feedForward(vector<vector<Neuron>> &prevLayer, vector<vector<double>> &fW, double &bias) {
	double sum = 0.0;
	for (unsigned sY = 0; sY < fW.size(); sY++) {
		for (unsigned sX = 0; sX < fW[0].size(); sX++) {
			sum += prevLayer[sY + posY][sX + posX].getOutputVal() * fW[sY][sX];
		}
	}
	sum += bias;
	n_outputVal = Neuron::transferFunction(sum);
}
Neuron::Neuron(unsigned numOutputs, unsigned myIndex) {
	for (unsigned c = 0; c < numOutputs; c++) {
		n_outputWeights.push_back(Connection());
		n_outputWeights.back().weight = randomWeight();
	}
	n_myIndex = myIndex;
	batchCount = 0;
}
Neuron::Neuron() {
	batchCount = 0;
}
Neuron::Neuron(unsigned pX, unsigned pY, int c) {
	posX = pX;
	posY = pY;
	batchCount = 0;
	biasGrad = 0;
}
Neuron::Neuron(vector<double> &outputWeights, unsigned myIndex) {
	n_myIndex = myIndex;
	batchCount = 0;
	for (unsigned c = 0; c < outputWeights.size(); c++) {
		n_outputWeights.push_back(Connection());
		n_outputWeights.back().weight = outputWeights[c];
	}
}
void Neuron::storeNeuron(vector<double> &weights) {
	weights.clear();
	for (int w = 0; w < n_outputWeights.size(); w++) {
		weights.push_back(n_outputWeights[w].weight);
		//weights.push_back(n_outputWeights[w].weight);
	}
}
void Neuron::updatePrevLayerGradient(vector<vector<Neuron>> &prevLayer, vector<vector<double>> &fW) {
	for (int y = 0; y < fW.size(); y++) {
		for (int x = 0; x < fW[y].size(); x++) {
			prevLayer[y + posY][x + posX].addGradient(n_gradient * fW[y][x]);
		}
	}
}